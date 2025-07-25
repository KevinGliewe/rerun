//! Rerun GUI theme and helpers, built around [`egui`](https://www.egui.rs/).

#![warn(clippy::iter_over_hash_type)] //  TODO(#6198): enable everywhere

pub mod alert;
mod color_table;
mod command;
mod command_palette;
mod context_ext;
mod design_tokens;
pub mod drag_and_drop;
pub mod filter_widget;
mod help;
mod hot_reload_design_tokens;
mod icon_text;
pub mod icons;
pub mod list_item;
mod markdown_utils;
pub mod modal;
pub mod notifications;
mod section_collapsing_header;
pub mod syntax_highlighting;
mod time_drag_value;
mod ui_ext;
mod ui_layout;

use egui::NumExt as _;

pub use self::{
    command::{UICommand, UICommandSender},
    command_palette::CommandPalette,
    context_ext::ContextExt,
    design_tokens::{DesignTokens, TableStyle},
    help::*,
    hot_reload_design_tokens::design_tokens_of,
    icon_text::*,
    icons::Icon,
    markdown_utils::*,
    section_collapsing_header::SectionCollapsingHeader,
    syntax_highlighting::SyntaxHighlighting,
    time_drag_value::TimeDragValue,
    ui_ext::UiExt,
    ui_layout::UiLayout,
};

#[cfg(feature = "arrow")]
mod arrow_ui;
pub mod menu;
pub mod time;

#[cfg(feature = "arrow")]
pub use self::arrow_ui::arrow_ui;

// ---------------------------------------------------------------------------

/// If true, we fill the entire window, except for the close/maximize/minimize buttons in the top-left.
/// See <https://github.com/emilk/egui/pull/2049>
pub const FULLSIZE_CONTENT: bool = cfg!(target_os = "macos");

/// If true, we hide the native window decoration
/// (the top bar with app title, close button etc),
/// and instead paint our own close/maximize/minimize buttons.
pub const CUSTOM_WINDOW_DECORATIONS: bool = false; // !FULLSIZE_CONTENT; // TODO(emilk): https://github.com/rerun-io/rerun/issues/1063

/// If true, we show the native window decorations/chrome with the
/// close/maximize/minimize buttons and app title.
pub const NATIVE_WINDOW_BAR: bool = !FULLSIZE_CONTENT && !CUSTOM_WINDOW_DECORATIONS;

// ----------------------------------------------------------------------------

pub struct TopBarStyle {
    /// Height of the top bar
    pub height: f32,

    /// Extra horizontal space in the top left corner to make room for
    /// close/minimize/maximize buttons (on Mac)
    pub indent: f32,
}

/// The style of a label.
///
/// This should be used for all UI widgets that support these styles.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelStyle {
    /// Regular style for a label.
    #[default]
    Normal,

    /// Label displaying the placeholder text for a yet unnamed item (e.g. an unnamed view).
    Unnamed,
}

// ----------------------------------------------------------------------------

pub fn design_tokens_of_visuals(visuals: &egui::Visuals) -> &'static DesignTokens {
    if visuals.dark_mode {
        design_tokens_of(egui::Theme::Dark)
    } else {
        design_tokens_of(egui::Theme::Light)
    }
}

/// Apply the Rerun design tokens to the given egui context and install image loaders.
pub fn apply_style_and_install_loaders(egui_ctx: &egui::Context) {
    re_tracing::profile_function!();

    egui_extras::install_image_loaders(egui_ctx);

    egui_ctx.include_bytes(
        "bytes://logo_dark_mode",
        include_bytes!("../data/logo_dark_mode.png"),
    );
    egui_ctx.include_bytes(
        "bytes://logo_light_mode",
        include_bytes!("../data/logo_light_mode.png"),
    );

    egui_ctx.options_mut(|o| {
        o.fallback_theme = egui::Theme::Dark; // If we don't know the system theme, use this as fallback
    });

    set_themes(egui_ctx);

    #[cfg(hot_reload_design_tokens)]
    {
        let egui_ctx = egui_ctx.clone();
        hot_reload_design_tokens::install_hot_reload(move || {
            re_log::debug!("Hot-reloading design tokens…");
            hot_reload_design_tokens::hot_reload_design_tokens();
            set_themes(&egui_ctx);
            egui_ctx.request_repaint();
        });
    }
}

fn set_themes(egui_ctx: &egui::Context) {
    // It's the same fonts in dark/light mode:
    design_tokens_of(egui::Theme::Dark).set_fonts(egui_ctx);

    for theme in [egui::Theme::Dark, egui::Theme::Light] {
        let mut style = std::sync::Arc::unwrap_or_clone(egui_ctx.style_of(theme));
        design_tokens_of(theme).apply(&mut style);
        egui_ctx.set_style_of(theme, style);
    }
}

fn format_with_decimals_in_range(
    value: f64,
    decimal_range: std::ops::RangeInclusive<usize>,
) -> String {
    fn format_with_decimals(value: f64, decimals: usize) -> String {
        re_format::FloatFormatOptions::DEFAULT_f64
            .with_decimals(decimals)
            .with_strip_trailing_zeros(false)
            .format(value)
    }

    let epsilon = 16.0 * f32::EPSILON; // margin large enough to handle most peoples round-tripping needs

    let min_decimals = *decimal_range.start();
    let max_decimals = *decimal_range.end();
    debug_assert!(min_decimals <= max_decimals);
    debug_assert!(max_decimals < 100);
    let max_decimals = max_decimals.at_most(16);
    let min_decimals = min_decimals.at_most(max_decimals);

    if min_decimals < max_decimals {
        // Try using a few decimals as possible, and then add more until we have enough precision
        // to round-trip the number.
        for decimals in min_decimals..max_decimals {
            let text = format_with_decimals(value, decimals);
            if let Some(parsed) = re_format::parse_f64(&text) {
                if egui::emath::almost_equal(parsed as f32, value as f32, epsilon) {
                    // Enough precision to show the value accurately - good!
                    return text;
                }
            }
        }
        // The value has more precision than we expected.
        // Probably the value was set not by the slider, but from outside.
        // In any case: show the full value
    }

    // Use max decimals
    format_with_decimals(value, max_decimals)
}

/// Is this Ui in a resizable panel?
///
/// Used as a heuristic to figure out if it is safe to truncate text.
///
/// In a resizable panel, it is safe to truncate text if it doesn't fit,
/// because the user can just make the panel wider to see the full text.
///
/// In other places, we should never truncate text, because then the user
/// cannot read it all. In those places (when this functions returns `false`)
/// you should either wrap the text or let it grow the Ui it is in.
fn is_in_resizable_panel(ui: &egui::Ui) -> bool {
    re_tracing::profile_function!();

    let mut is_in_side_panel = false;

    for frame in ui.stack().iter() {
        if let Some(kind) = frame.kind() {
            if kind.is_area() {
                return false; // Our popups (tooltips etc) aren't resizable
            }
            if matches!(kind, egui::UiKind::LeftPanel | egui::UiKind::RightPanel) {
                is_in_side_panel = true;
            }
        }
    }

    if is_in_side_panel {
        true // Our side-panels are resizable
    } else {
        false // Safe fallback
    }
}
