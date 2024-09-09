## `Point2d`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::num::f32`),
]
```

## ``Point2d`::#derive`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d::#derive(0)`),
    ItemPath(`core::fmt::Debug`),
    ItemPath(`core::clone::Clone`),
    ItemPath(`core::visual::Visualize`),
]
```

## `RelativePoint2d`

```rust
[
    ItemPath(`mnist_classifier::geom2d::RelativePoint2d`),
    ItemPath(`core::num::f32`),
]
```

## `Vector2d`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32`),
]
```

## `ClosedRange`

```rust
[
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `BoundingBox`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `RelativeBoundingBox`

```rust
[
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `impl Point2d`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d(0)`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::num::f32`),
]
```

## `Point2d::from_i_shift28`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::from_i_shift28`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::num::f32`),
]
```

## `Point2d::vector`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::vector`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32`),
]
```

## `Point2d::to`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32`),
]
```

## `Point2d::norm`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::norm`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::num::f32(0)::sqrt`),
]
```

## `Point2d::dist`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::dist`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32(0)::sqrt`),
]
```

## `impl Vector2d`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32`),
]
```

## `Vector2d::point`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::point`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::num::f32`),
]
```

## `Vector2d::to`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::to`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32`),
]
```

## `Vector2d::norm`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::num::f32(0)::sqrt`),
]
```

## `Vector2d::dot`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32`),
]
```

## `Vector2d::cross`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::f32`),
]
```

## `Vector2d::angle`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle`),
    ItemPath(`core::basic::bool`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
    ItemPath(`core::num::f32(0)::min`),
    ItemPath(`core::num::f32(0)::sgnx`),
    ItemPath(`core::num::f32(0)::acos`),
    ItemPath(`core::num::f32(0)::sqrt`),
    ItemPath(`core::num::i32`),
]
```

## `Vector2d::rotation_direction_to`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
    ItemPath(`core::num::f32(0)::sgnx`),
    ItemPath(`core::num::f32`),
]
```

## `Vector2d::angle_to`

```rust
[
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle_to`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::basic::bool`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
    ItemPath(`core::num::f32(0)::min`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
    ItemPath(`core::num::f32(0)::acos`),
    ItemPath(`core::num::f32(0)::sqrt`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
    ItemPath(`core::num::f32(0)::sgnx`),
]
```

## `impl ClosedRange`

```rust
[
    ItemPath(`mnist_classifier::geom2d::ClosedRange(0)`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `ClosedRange::relative_range`

```rust
[
    ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_range`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `ClosedRange::relative_point`

```rust
[
    ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_point`),
    ItemPath(`core::num::f32`),
]
```

## `impl BoundingBox`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)`),
    ItemPath(`mnist_classifier::geom2d::BoundingBox`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `BoundingBox::relative_bounding_box`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
    ItemPath(`mnist_classifier::geom2d::BoundingBox`),
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_range`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `BoundingBox::relative_point`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_point`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist_classifier::geom2d::RelativePoint2d`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_point`),
    ItemPath(`core::num::f32`),
]
```

## `BoundingBox::xmin`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmin`),
    ItemPath(`core::num::f32`),
]
```

## `BoundingBox::xmax`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::xmax`),
    ItemPath(`core::num::f32`),
]
```

## `BoundingBox::ymin`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymin`),
    ItemPath(`core::num::f32`),
]
```

## `BoundingBox::ymax`

```rust
[
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::ymax`),
    ItemPath(`core::num::f32`),
]
```

## `impl RelativeBoundingBox`

```rust
[
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)`),
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::f32`),
]
```

## `RelativeBoundingBox::xmin`

```rust
[
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`),
    ItemPath(`core::num::f32`),
]
```

## `RelativeBoundingBox::xmax`

```rust
[
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`),
    ItemPath(`core::num::f32`),
]
```

## `RelativeBoundingBox::ymin`

```rust
[
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`),
    ItemPath(`core::num::f32`),
]
```

## `RelativeBoundingBox::ymax`

```rust
[
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`),
    ItemPath(`core::num::f32`),
]
```
