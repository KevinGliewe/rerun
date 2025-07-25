---
title: "FillMode"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/website.rs -->

How a geometric shape is drawn and colored.

## Variants
#### `MajorWireframe` = 1
Lines are drawn around the parts of the shape which directly correspond to the logged data.

Examples of what this means:

* An [`archetypes.Ellipsoids3D`](https://rerun.io/docs/reference/types/archetypes/ellipsoids3d) will draw three axis-aligned ellipses that are cross-sections
  of each ellipsoid, each of which displays two out of three of the sizes of the ellipsoid.
* For [`archetypes.Boxes3D`](https://rerun.io/docs/reference/types/archetypes/boxes3d), it is the edges of the box, identical to [`components.FillMode#DenseWireframe`](https://rerun.io/docs/reference/types/components/fill_mode).

#### `DenseWireframe` = 2
Many lines are drawn to represent the surface of the shape in a see-through fashion.

Examples of what this means:

* An [`archetypes.Ellipsoids3D`](https://rerun.io/docs/reference/types/archetypes/ellipsoids3d) will draw a wireframe triangle mesh that approximates each
  ellipsoid.
* For [`archetypes.Boxes3D`](https://rerun.io/docs/reference/types/archetypes/boxes3d), it is the edges of the box, identical to [`components.FillMode#MajorWireframe`](https://rerun.io/docs/reference/types/components/fill_mode).

#### `Solid` = 3
The surface of the shape is filled in with a solid color. No lines are drawn.


## Arrow datatype
```
uint8
```

## API reference links
 * 🌊 [C++ API docs for `FillMode`](https://ref.rerun.io/docs/cpp/stable/namespacererun_1_1components.html)
 * 🐍 [Python API docs for `FillMode`](https://ref.rerun.io/docs/python/stable/common/components#rerun.components.FillMode)
 * 🦀 [Rust API docs for `FillMode`](https://docs.rs/rerun/latest/rerun/components/enum.FillMode.html)


## Used by

* [`Boxes3D`](../archetypes/boxes3d.md)
* [`Capsules3D`](../archetypes/capsules3d.md)
* [`Cylinders3D`](../archetypes/cylinders3d.md)
* [`Ellipsoids3D`](../archetypes/ellipsoids3d.md)
