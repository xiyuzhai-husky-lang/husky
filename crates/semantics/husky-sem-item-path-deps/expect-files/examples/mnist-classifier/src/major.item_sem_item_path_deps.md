## `connected_components`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
            ItemPath(`mnist_classifier::connected_component::find_connected_components`),
            ItemPath(`mnist::INPUT`),
        ],
    ),
)
```

## `major_connected_component`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
            ItemPath(`mnist_classifier::major::connected_components`),
            ItemPath(`core::vec::Vec(0)::ilen`),
        ],
    ),
)
```

## `ignored_connected_components_row_span_sum_sum`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::major::connected_components`),
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`mnist_classifier::major::major_connected_component`),
        ],
    ),
)
```

## `major_raw_contours`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::major::major_connected_component`),
        ],
    ),
)
```

## `major_raw_contour`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::major::major_connected_component`),
        ],
    ),
)
```

## `major_line_segment_sketch`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
            ItemPath(`mnist_classifier::major::major_raw_contour`),
        ],
    ),
)
```

## `major_concave_components`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`mnist_classifier::major::major_line_segment_sketch`),
        ],
    ),
)
```
