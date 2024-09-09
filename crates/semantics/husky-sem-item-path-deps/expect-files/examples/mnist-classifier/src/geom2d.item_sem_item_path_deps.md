## `Point2d`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## ``Point2d`::#derive`

```rust
Some(
    Ok(
        [
            ItemPath(`core::fmt::Debug`),
            ItemPath(`core::clone::Clone`),
            ItemPath(`core::visual::Visualize`),
        ],
    ),
)
```

## `RelativePoint2d`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `Vector2d`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `ClosedRange`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `BoundingBox`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::ClosedRange`),
        ],
    ),
)
```

## `RelativeBoundingBox`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::ClosedRange`),
        ],
    ),
)
```

## `impl Point2d`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
        ],
    ),
)
```

## `Point2d::from_i_shift28`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::i32`),
            ItemPath(`mnist_classifier::geom2d::Point2d`),
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `Point2d::vector`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
        ],
    ),
)
```

## `Point2d::to`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
        ],
    ),
)
```

## `Point2d::norm`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`core::num::f32(0)::sqrt`),
        ],
    ),
)
```

## `Point2d::dist`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
        ],
    ),
)
```

## `impl Vector2d`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
        ],
    ),
)
```

## `Vector2d::point`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
        ],
    ),
)
```

## `Vector2d::to`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
        ],
    ),
)
```

## `Vector2d::norm`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`core::num::f32(0)::sqrt`),
        ],
    ),
)
```

## `Vector2d::dot`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `Vector2d::cross`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `Vector2d::angle`

```rust
Some(
    Ok(
        [
            ItemPath(`core::basic::bool`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
            ItemPath(`core::num::f32(0)::min`),
            ItemPath(`core::num::f32(0)::sgnx`),
            ItemPath(`core::num::f32(0)::acos`),
        ],
    ),
)
```

## `Vector2d::rotation_direction_to`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`core::num::i32`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
            ItemPath(`core::num::f32(0)::sgnx`),
        ],
    ),
)
```

## `Vector2d::angle_to`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`core::basic::bool`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
            ItemPath(`core::num::f32(0)::min`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
            ItemPath(`core::num::f32(0)::acos`),
        ],
    ),
)
```

## `impl ClosedRange`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::ClosedRange`),
        ],
    ),
)
```

## `ClosedRange::relative_range`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::ClosedRange`),
        ],
    ),
)
```

## `ClosedRange::relative_point`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `impl BoundingBox`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::BoundingBox`),
        ],
    ),
)
```

## `BoundingBox::relative_bounding_box`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::BoundingBox`),
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
            ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_range`),
        ],
    ),
)
```

## `BoundingBox::relative_point`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
            ItemPath(`mnist_classifier::geom2d::RelativePoint2d`),
            ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_point`),
        ],
    ),
)
```

## `BoundingBox::xmin`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `BoundingBox::xmax`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `BoundingBox::ymin`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `BoundingBox::ymax`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `impl RelativeBoundingBox`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
        ],
    ),
)
```

## `RelativeBoundingBox::xmin`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `RelativeBoundingBox::xmax`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `RelativeBoundingBox::ymin`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `RelativeBoundingBox::ymax`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```
