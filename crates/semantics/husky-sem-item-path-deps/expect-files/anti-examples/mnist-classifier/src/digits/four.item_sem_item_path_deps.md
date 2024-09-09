## `left_components`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
            ItemPath(`mnist_classifier::fermi::fermi_match`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
        ],
    ),
)
```

## `left_coordinate_max`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`),
        ],
    ),
)
```

## `components_max_downwards`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
            ItemPath(`mnist_classifier::fermi::fermi_match`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
        ],
    ),
)
```

## `components_max_heights`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
            ItemPath(`mnist_classifier::fermi::fermi_match`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
        ],
    ),
)
```

## `is_four`

```rust
Some(
    Ok(
        [
            ItemPath(`malamute::OneVsAll`),
            ItemPath(`mnist::MnistLabel`),
            ItemPath(`mnist::MnistLabel::Four`),
            ItemPath(`mnist_classifier::digits::four::left_components`),
            ItemPath(`mnist_classifier::major::major_connected_component`),
            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`),
            ItemPath(`malamute::OneVsAll::Yes`),
        ],
    ),
)
```

## `displacement_downwards`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
        ],
    ),
)
```

## `cc_box_heights`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`),
        ],
    ),
)
```
