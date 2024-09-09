## `RawContour`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
            ItemPath(`mnist_classifier::geom2d::Point2d`),
        ],
    ),
)
```

## `Direction`

```rust
Some(
    Ok(
        [],
    ),
)
```

## ``Direction`::#derive`

```rust
Some(
    Ok(
        [
            ItemPath(`core::clone::Clone`),
            ItemPath(`core::marker::Copy`),
            ItemPath(`core::cmp::PartialEq`),
            ItemPath(`core::cmp::Eq`),
        ],
    ),
)
```

## `Direction::Up`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `Direction::Left`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `Direction::Down`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `Direction::Right`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `get_pixel_pair`

```rust
Some(
    Ok(
        [
            ItemPath(`core::raw_bits::r32`),
            ItemPath(`core::num::i32`),
        ],
    ),
)
```

## `get_pixel_to_the_left`

```rust
Some(
    Ok(
        [
            ItemPath(`core::raw_bits::r32`),
            ItemPath(`core::num::i32`),
        ],
    ),
)
```

## `get_pixel_to_the_right`

```rust
Some(
    Ok(
        [
            ItemPath(`core::raw_bits::r32`),
            ItemPath(`core::num::i32`),
        ],
    ),
)
```

## `get_inward_direction`

```rust
Some(
    Ok(
        [
            ItemPath(`core::raw_bits::r32`),
            ItemPath(`core::num::i32`),
            ItemPath(`mnist_classifier::raw_contour::Direction`),
            ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
        ],
    ),
)
```

## `get_angle_change`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::Direction`),
            ItemPath(`core::num::i32`),
            ItemPath(`core::raw_bits::r32`),
            ItemPath(`core::raw_bits::r32(0)::last_bits`),
        ],
    ),
)
```

## `get_outward_direction`

```rust
Some(
    Ok(
        [
            ItemPath(`core::raw_bits::r32`),
            ItemPath(`core::num::i32`),
            ItemPath(`mnist_classifier::raw_contour::Direction`),
            ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
            ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
        ],
    ),
)
```

## `StreakCache`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::i32`),
        ],
    ),
)
```

## `get_concave_middle_point`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
            ItemPath(`core::vec::Vec(0)::ilen`),
        ],
    ),
)
```

## `find_raw_contours`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist::BinaryGrid28(0)::new_zeros`),
            ItemPath(`mnist_classifier::geom2d::Point2d`),
            ItemPath(`core::raw_bits::r32(0)::ctz`),
            ItemPath(`mnist_classifier::raw_contour::get_inward_direction`),
            ItemPath(`mnist_classifier::raw_contour::get_outward_direction`),
            ItemPath(`mnist_classifier::raw_contour::get_angle_change`),
            ItemPath(`core::vec::Vec(0)::last`),
            ItemPath(`mnist_classifier::raw_contour::get_concave_middle_point`),
            ItemPath(`mnist_classifier::geom2d::Point2d(0)::from_i_shift28`),
            ItemPath(`core::vec::Vec(0)::push`),
            ItemPath(`core::vec::Vec(0)::pop`),
        ],
    ),
)
```

## `impl Visualize for RawContour`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visualize`),
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
        ],
    ),
)
```

## `(RawContour as Visualize)::visualize`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visual`),
        ],
    ),
)
```

## `impl RawContour`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
        ],
    ),
)
```

## `RawContour::line_segment_sketch`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
        ],
    ),
)
```

## `RawContour::bounding_box`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::BoundingBox`),
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`core::num::f32(0)::min`),
            ItemPath(`core::num::f32(0)::max`),
            ItemPath(`mnist_classifier::geom2d::ClosedRange`),
        ],
    ),
)
```

## `RawContour::relative_bounding_box`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
            ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
        ],
    ),
)
```

## `RawContour::contour_len`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`core::num::f32(0)::abs`),
        ],
    ),
)
```

## `RawContour::displacement`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::i32`),
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
        ],
    ),
)
```
