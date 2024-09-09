## `simple_seven_match`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
            ItemPath(`mnist_classifier::fermi::fermi_match`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`),
        ],
    ),
)
```

## `simple_leftdown_pattern`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
        ],
    ),
)
```

## `special_seven_match`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
            ItemPath(`mnist_classifier::fermi::fermi_match`),
            ItemPath(`mnist_classifier::major::major_concave_components`),
            ItemPath(`mnist_classifier::digits::seven::leftupcc_pattern`),
            ItemPath(`mnist_classifier::digits::seven::leftdowncc_pattern`),
        ],
    ),
)
```

## `leftupcc_pattern`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
        ],
    ),
)
```

## `leftdowncc_pattern`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle`),
        ],
    ),
)
```

## `is_seven`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::major::major_connected_component`),
            ItemPath(`mnist_classifier::digits::seven::simple_seven_match`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`),
            ItemPath(`mnist_classifier::digits::seven::special_seven_match`),
        ],
    ),
)
```
