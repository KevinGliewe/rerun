#![allow(non_snake_case)]

use crate::{components, view_coordinates::ViewDir};

use super::ViewCoordinates;

macro_rules! define_coordinates {
    ($docstring:literal, $name:ident => ($x:ident, $y:ident, $z:ident) ) => {
        #[doc = $docstring]
        pub fn $name() -> Self {
            Self::new(components::ViewCoordinates::new(
                ViewDir::$x,
                ViewDir::$y,
                ViewDir::$z,
            ))
        }
    };
}

impl ViewCoordinates {
    // <BEGIN_GENERATED:declarations>
    // This section is generated by running `scripts/generate_view_coordinate_defs.py --rust`
    define_coordinates!("X=Up, Y=Left, Z=Forward

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", ULF => (Up, Left, Forward));
    define_coordinates!("X=Up, Y=Forward, Z=Left", UFL => (Up, Forward, Left));
    define_coordinates!("X=Left, Y=Up, Z=Forward", LUF => (Left, Up, Forward));
    define_coordinates!("X=Left, Y=Forward, Z=Up

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LFU => (Left, Forward, Up));
    define_coordinates!("X=Forward, Y=Up, Z=Left

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", FUL => (Forward, Up, Left));
    define_coordinates!("X=Forward, Y=Left, Z=Up", FLU => (Forward, Left, Up));
    define_coordinates!("X=Up, Y=Left, Z=Back", ULB => (Up, Left, Back));
    define_coordinates!("X=Up, Y=Back, Z=Left

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", UBL => (Up, Back, Left));
    define_coordinates!("X=Left, Y=Up, Z=Back

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LUB => (Left, Up, Back));
    define_coordinates!("X=Left, Y=Back, Z=Up", LBU => (Left, Back, Up));
    define_coordinates!("X=Back, Y=Up, Z=Left", BUL => (Back, Up, Left));
    define_coordinates!("X=Back, Y=Left, Z=Up

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", BLU => (Back, Left, Up));
    define_coordinates!("X=Up, Y=Right, Z=Forward", URF => (Up, Right, Forward));
    define_coordinates!("X=Up, Y=Forward, Z=Right

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", UFR => (Up, Forward, Right));
    define_coordinates!("X=Right, Y=Up, Z=Forward

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", RUF => (Right, Up, Forward));
    define_coordinates!("X=Right, Y=Forward, Z=Up", RFU => (Right, Forward, Up));
    define_coordinates!("X=Forward, Y=Up, Z=Right", FUR => (Forward, Up, Right));
    define_coordinates!("X=Forward, Y=Right, Z=Up

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", FRU => (Forward, Right, Up));
    define_coordinates!("X=Up, Y=Right, Z=Back

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", URB => (Up, Right, Back));
    define_coordinates!("X=Up, Y=Back, Z=Right", UBR => (Up, Back, Right));
    define_coordinates!("X=Right, Y=Up, Z=Back", RUB => (Right, Up, Back));
    define_coordinates!("X=Right, Y=Back, Z=Up

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", RBU => (Right, Back, Up));
    define_coordinates!("X=Back, Y=Up, Z=Right

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", BUR => (Back, Up, Right));
    define_coordinates!("X=Back, Y=Right, Z=Up", BRU => (Back, Right, Up));
    define_coordinates!("X=Down, Y=Left, Z=Forward", DLF => (Down, Left, Forward));
    define_coordinates!("X=Down, Y=Forward, Z=Left

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", DFL => (Down, Forward, Left));
    define_coordinates!("X=Left, Y=Down, Z=Forward

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LDF => (Left, Down, Forward));
    define_coordinates!("X=Left, Y=Forward, Z=Down", LFD => (Left, Forward, Down));
    define_coordinates!("X=Forward, Y=Down, Z=Left", FDL => (Forward, Down, Left));
    define_coordinates!("X=Forward, Y=Left, Z=Down

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", FLD => (Forward, Left, Down));
    define_coordinates!("X=Down, Y=Left, Z=Back

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", DLB => (Down, Left, Back));
    define_coordinates!("X=Down, Y=Back, Z=Left", DBL => (Down, Back, Left));
    define_coordinates!("X=Left, Y=Down, Z=Back", LDB => (Left, Down, Back));
    define_coordinates!("X=Left, Y=Back, Z=Down

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LBD => (Left, Back, Down));
    define_coordinates!("X=Back, Y=Down, Z=Left

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", BDL => (Back, Down, Left));
    define_coordinates!("X=Back, Y=Left, Z=Down", BLD => (Back, Left, Down));
    define_coordinates!("X=Down, Y=Right, Z=Forward

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", DRF => (Down, Right, Forward));
    define_coordinates!("X=Down, Y=Forward, Z=Right", DFR => (Down, Forward, Right));
    define_coordinates!("X=Right, Y=Down, Z=Forward", RDF => (Right, Down, Forward));
    define_coordinates!("X=Right, Y=Forward, Z=Down

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", RFD => (Right, Forward, Down));
    define_coordinates!("X=Forward, Y=Down, Z=Right

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", FDR => (Forward, Down, Right));
    define_coordinates!("X=Forward, Y=Right, Z=Down", FRD => (Forward, Right, Down));
    define_coordinates!("X=Down, Y=Right, Z=Back", DRB => (Down, Right, Back));
    define_coordinates!("X=Down, Y=Back, Z=Right

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", DBR => (Down, Back, Right));
    define_coordinates!("X=Right, Y=Down, Z=Back

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", RDB => (Right, Down, Back));
    define_coordinates!("X=Right, Y=Back, Z=Down", RBD => (Right, Back, Down));
    define_coordinates!("X=Back, Y=Down, Z=Right", BDR => (Back, Down, Right));
    define_coordinates!("X=Back, Y=Right, Z=Down

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", BRD => (Back, Right, Down));
    define_coordinates!("X=Up, Y=Right, Z=Forward", RIGHT_HAND_X_UP => (Up, Right, Forward));
    define_coordinates!("X=Down, Y=Right, Z=Back", RIGHT_HAND_X_DOWN => (Down, Right, Back));
    define_coordinates!("X=Right, Y=Up, Z=Back", RIGHT_HAND_Y_UP => (Right, Up, Back));
    define_coordinates!("X=Right, Y=Down, Z=Forward", RIGHT_HAND_Y_DOWN => (Right, Down, Forward));
    define_coordinates!("X=Right, Y=Forward, Z=Up", RIGHT_HAND_Z_UP => (Right, Forward, Up));
    define_coordinates!("X=Right, Y=Back, Z=Down", RIGHT_HAND_Z_DOWN => (Right, Back, Down));
    define_coordinates!("X=Up, Y=Right, Z=Back

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LEFT_HAND_X_UP => (Up, Right, Back));
    define_coordinates!("X=Down, Y=Right, Z=Forward

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LEFT_HAND_X_DOWN => (Down, Right, Forward));
    define_coordinates!("X=Right, Y=Up, Z=Forward

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LEFT_HAND_Y_UP => (Right, Up, Forward));
    define_coordinates!("X=Right, Y=Down, Z=Back

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LEFT_HAND_Y_DOWN => (Right, Down, Back));
    define_coordinates!("X=Right, Y=Back, Z=Up

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LEFT_HAND_Z_UP => (Right, Back, Up));
    define_coordinates!("X=Right, Y=Forward, Z=Down

⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).", LEFT_HAND_Z_DOWN => (Right, Forward, Down));
    // <END_GENERATED:declarations>
}
