## `ConnectedComponentDistribution`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
    ItemPath(`core::num::i32`),
]
```

## `EffHoles`

```rust
[
    ItemPath(`mnist_classifier::connected_component::EffHoles`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`core::num::f32`),
]
```

## `hole_tmpl`

```rust
[
    ItemPath(`mnist_classifier::connected_component::hole_tmpl`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist::BinaryImage28`),
]
```

## `ConnectedComponent`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist::BinaryImage28`),
]
```

## `horizontal_extend`

```rust
[
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `find_connected_components`

```rust
[
    ItemPath(`mnist_classifier::connected_component::find_connected_components`),
    ItemPath(`mnist::BinaryImage28`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
    ItemPath(`core::raw_bits::r32(0)::ctz`),
    ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
    ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::raw_bits::r32`),
]
```

## `impl Visualize for ConnectedComponent`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
    ItemPath(`core::visual::Visualize`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist::BinaryImage28`),
]
```

## `(ConnectedComponent as Visualize)::visualize`

```rust
[
    ItemPath(`<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`),
    ItemPath(`core::visual::Visual`),
    ItemPath(`<mnist::BinaryImage28 as core::visual::Visualize(0)>::visualize`),
]
```

## `impl ConnectedComponent`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist::BinaryImage28`),
]
```

## `ConnectedComponent::raw_contours`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::raw_contour::find_raw_contours`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`mnist::BinaryGrid28(0)::new_zeros`),
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

## `ConnectedComponent::eff_holes`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`),
    ItemPath(`mnist_classifier::connected_component::EffHoles`),
    ItemPath(`core::vec::Vec(0)::collect_leashes`),
    ItemPath(`mnist_classifier::raw_contour::RawContour`),
    ItemPath(`mnist_classifier::connected_component::hole_tmpl`),
    ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
    ItemPath(`core::vec::Vec(0)::push`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
    ItemPath(`mnist_classifier::geom2d::Point2d`),
    ItemPath(`core::num::f32`),
    ItemPath(`mnist::BinaryImage28`),
]
```

## `ConnectedComponent::max_hole_ilen`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::vec::Vec(0)::ilen`),
    ItemPath(`core::num::i32`),
]
```

## `ConnectedComponent::max_row_span`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::raw_bits::r32(0)::span`),
    ItemPath(`core::num::i32(0)::max`),
]
```

## `ConnectedComponent::row_span_sum`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::raw_bits::r32(0)::span`),
    ItemPath(`core::num::i32`),
]
```

## `ConnectedComponent::distribution`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::distribution`),
    ItemPath(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
    ItemPath(`core::raw_bits::r32(0)::co`),
    ItemPath(`core::num::i32`),
]
```

## `ConnectedComponent::upper_mass`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`),
    ItemPath(`core::num::f32`),
]
```

## `ConnectedComponent::lower_mass`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`),
    ItemPath(`core::num::f32`),
]
```

## `ConnectedComponent::top_k_row_span_sum`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::raw_bits::r32(0)::span`),
]
```

## `ConnectedComponent::top_k_row_right_mass_sum`

```rust
[
    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`),
    ItemPath(`core::num::i32`),
    ItemPath(`core::num::f32`),
    ItemPath(`core::raw_bits::r32(0)::right_mass`),
]
```
