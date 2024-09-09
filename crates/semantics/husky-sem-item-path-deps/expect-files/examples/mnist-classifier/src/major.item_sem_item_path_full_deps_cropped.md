## `connected_components`

```rust
[
    ItemPath(`mnist_classifier::major::connected_components`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::INPUT`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `major_connected_component`

```rust
[
    ItemPath(`mnist_classifier::major::major_connected_component`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::major::connected_components`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::INPUT`),
    ItemPath(`core::num::i32`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `ignored_connected_components_row_span_sum_sum`

```rust
[
    ItemPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::major::connected_components`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist_classifier::major::major_connected_component`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::INPUT`),
    ItemPath(`core::num::i32`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `major_raw_contours`

```rust
[
    ItemPath(`mnist_classifier::major::major_raw_contours`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::major::major_connected_component`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist_classifier::major::connected_components`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::INPUT`),
    ItemPath(`core::num::i32`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `major_raw_contour`

```rust
[
    ItemPath(`mnist_classifier::major::major_raw_contour`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::major::major_connected_component`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist_classifier::major::connected_components`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::INPUT`),
    ItemPath(`core::num::i32`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `major_line_segment_sketch`

```rust
[
    ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
    ItemPath(`mnist_classifier::major::major_raw_contour`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
    ItemPath(`mnist_classifier::major::major_connected_component`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::slice::CyclicSlice`),
    ItemPath(`core::slice::CyclicSlice(0)::first`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::slice::CyclicSlice(0)::last`),
    ItemPath(`mnist_classifier::major::connected_components`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::INPUT`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `major_concave_components`

```rust
[
    ItemPath(`mnist_classifier::major::major_concave_components`),
    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
    ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
    ItemPath(`core::slice::CyclicSlice`),
    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
    ItemPath(`mnist_classifier::major::major_raw_contour`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::slice::CyclicSlice(0)::first`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::slice::CyclicSlice(0)::last`),
    ItemPath(`mnist_classifier::major::major_connected_component`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::major::connected_components`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::INPUT`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::raw_bits::r32`),
]
```
