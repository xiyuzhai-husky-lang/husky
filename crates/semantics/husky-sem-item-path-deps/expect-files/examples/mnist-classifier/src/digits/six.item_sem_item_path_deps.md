## `six_match`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
            ItemPath(`mnist_classifier::fermi::fermi_match`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`mnist_classifier::digits::six::upmost`),
        ],
    ),
)
```

## `six_match_refined1`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
            ItemPath(`mnist_classifier::fermi::fermi_match`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`mnist_classifier::digits::six::upmost`),
            ItemPath(`mnist_classifier::digits::six::bottom1`),
        ],
    ),
)
```

## `is_six`

```rust
Some(
    Ok(
        [
            ItemPath(`malamute::OneVsAll`),
            ItemPath(`mnist::MnistLabel`),
            ItemPath(`mnist::MnistLabel::Six`),
            ItemPath(`mnist_classifier::digits::six::six_match`),
            ItemPath(`mnist_classifier::major::major_connected_component`),
            ItemPath(`malamute::narrow_down`),
            ItemPath(`mnist_classifier::digits::six::six_match_refined1`),
            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
            ItemPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`),
            ItemPath(`malamute::OneVsAll::Yes`),
            ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
            ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_point`),
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`),
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`),
        ],
    ),
)
```

## `upmost`

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

## `bottom1`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
            ItemPath(`core::num::f32(0)::abs`),
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
            ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_point`),
        ],
    ),
)
```
