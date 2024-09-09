## `RawContour`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
]
```

## `Direction`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::Direction`),
]
```

## ``Direction`::#derive`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::Direction::#derive(0)`),
    ItemPath(`core::clone::Clone`),
    ItemPath(`core::marker::Copy`),
    ItemPath(`core::cmp::PartialEq`),
    ItemPath(`core::cmp::Eq`),
]
```

## `Direction::Up`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
]
```

## `Direction::Left`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
]
```

## `Direction::Down`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
]
```

## `Direction::Right`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
]
```

## `get_pixel_pair`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
    ItemPath(`core::raw_bits::r32`),
    ItemPath(`core::num::i32`),
]
```

## `get_pixel_to_the_left`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`),
    ItemPath(`core::raw_bits::r32`),
    ItemPath(`core::num::i32`),
]
```

## `get_pixel_to_the_right`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`),
    ItemPath(`core::raw_bits::r32`),
    ItemPath(`core::num::i32`),
]
```

## `get_inward_direction`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::get_inward_direction`),
    ItemPath(`core::raw_bits::r32`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist_classifier::raw_contour::Direction`),
    ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
]
```

## `get_angle_change`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::get_angle_change`),
    ItemPath(`mnist_classifier::raw_contour::Direction`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::raw_bits::r32`),
    ItemPath(`core::raw_bits::r32(0)::last_bits`),
]
```

## `get_outward_direction`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::get_outward_direction`),
    ItemPath(`core::raw_bits::r32`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist_classifier::raw_contour::Direction`),
    ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
]
```

## `StreakCache`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::StreakCache`),
    ItemPath(`core::num::i32`),
]
```

## `get_concave_middle_point`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::get_concave_middle_point`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::num::i32`),
]
```

## `find_raw_contours`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::find_raw_contours`),
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
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::raw_bits::r32`),
    ItemPath(`mnist_classifier::raw_contour::Direction`),
    ItemPath(`mnist_classifier::raw_contour::get_pixel_pair`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
    ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
    ItemPath(`core::raw_bits::r32(0)::last_bits`),
    ItemPath(`core::option::Option`),
    ItemPath(`core::vec::Vec(0)::ilen`),
]
```

## `impl Visualize for RawContour`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
    ItemPath(`core::visual::Visualize`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
]
```

## `(RawContour as Visualize)::visualize`

```rust
[
    ItemPath(`<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`),
    ItemPath(`core::visual::Visual`),
]
```

## `impl RawContour`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour(0)`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
]
```

## `RawContour::line_segment_sketch`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::line_segment_sketch::find_line_segments`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::slice::CyclicSlice`),
    ItemPath(`core::slice::CyclicSlice(0)::first`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::slice::CyclicSlice(0)::last`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist_classifier::line_segment_sketch::extend_end`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
    ItemPath(`core::vec::Vec(0)::last`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
    ItemPath(`core::num::f32(0)::abs`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
    ItemPath(`core::slice::CyclicSlice(0)::start`),
    ItemPath(`mnist_classifier::line_segment_sketch::extend_start`),
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
    ItemPath(`core::vec::Vec(0)::pop`),
    ItemPath(`core::slice::CyclicSlice(0)::end`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::vec::Vec(0)::first`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
    ItemPath(`mnist_classifier::line_segment_sketch::go_right`),
    ItemPath(`mnist_classifier::line_segment_sketch::go_left`),
    ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
    ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::option::Option`),
    ItemPath(`core::num::i32(0)::min`),
    ItemPath(`core::num::f32(0)::sqrt`),
    ItemPath(`core::num::f32(0)::sgnx`),
]
```

## `RawContour::bounding_box`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::bounding_box`),
    ItemPath(`mnist_classifier::geom2d::BoundingBox`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`core::num::f32(0)::min`),
    ItemPath(`core::num::f32(0)::max`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::num::f32`),
]
```

## `RawContour::relative_bounding_box`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`),
    ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
    ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange`),
    ItemPath(`mnist_classifier::geom2d::BoundingBox`),
    ItemPath(`mnist_classifier::geom2d::ClosedRange(0)::relative_range`),
    ItemPath(`core::num::f32`),
]
```

## `RawContour::contour_len`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::contour_len`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`core::num::f32(0)::abs`),
    ItemPath(`core::num::i32`),
]
```

## `RawContour::displacement`

```rust
[
    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist_classifier::geom2d::Vector2d`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
]
```
