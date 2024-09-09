## `connected_components` decl

```rust
Some(
    [
        "[",
        "ConnectedComponent",
        "[]ConnectedComponent",
    ],
)
```

## `connected_components` defn

```rust
Some(
    [
        "find_connected_components",
        "INPUT",
        "find_connected_components(INPUT)",
        "find_connected_components(INPUT)",
        "find_connected_components(INPUT)",
    ],
)
```

## `major_connected_component` decl

```rust
Some(
    [
        "ConnectedComponent",
        "~ConnectedComponent",
    ],
)
```

## `major_connected_component` defn

```rust
Some(
    [
        "0",
        "let mut i0 = 0",
        "0.0",
        "let mut max_row_span_sum = 0.0",
        "connected_components",
        "connected_components.ilen()",
        "i",
        "connected_components",
        "i",
        "connected_components[i]",
        "connected_components[i].row_span_sum",
        "let row_span_sum = connected_components[i].row_span_sum",
        "row_span_sum",
        "max_row_span_sum",
        "row_span_sum > max_row_span_sum",
        "row_span_sum > max_row_span_sum",
        "max_row_span_sum",
        "row_span_sum",
        "max_row_span_sum = row_span_sum",
        "max_row_span_sum = row_span_sum",
        "i0",
        "i",
        "i0 = i",
        "i0 = i",
        "if row_span_sum > max_row_span_sum:\n            max_row_span_sum = row_span_sum\n            i0 = i",
        "for i < connected_components.ilen():\n        let row_span_sum = connected_components[i].row_span_sum\n        if row_span_sum > max_row_span_sum:\n            max_row_span_sum = row_span_sum\n            i0 = i",
        "connected_components",
        "i0",
        "connected_components[i0]",
        "return connected_components[i0]",
        "let mut i0 = 0\n    let mut max_row_span_sum = 0.0\n    for i < connected_components.ilen():\n        let row_span_sum = connected_components[i].row_span_sum\n        if row_span_sum > max_row_span_sum:\n            max_row_span_sum = row_span_sum\n            i0 = i\n    return connected_components[i0]",
    ],
)
```

## `ignored_connected_components_row_span_sum_sum` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ignored_connected_components_row_span_sum_sum` defn

```rust
Some(
    [
        "0.0",
        "let mut sum = 0.0",
        "connected_components",
        "connected_components.ilen()",
        "i",
        "sum",
        "connected_components",
        "i",
        "connected_components[i]",
        "connected_components[i].row_span_sum",
        "sum += connected_components[i].row_span_sum",
        "sum += connected_components[i].row_span_sum",
        "for i < connected_components.ilen():\n        sum += connected_components[i].row_span_sum",
        "sum",
        "major_connected_component",
        "major_connected_component.row_span_sum",
        "sum - major_connected_component.row_span_sum",
        "return sum - major_connected_component.row_span_sum",
        "let mut sum = 0.0\n    for i < connected_components.ilen():\n        sum += connected_components[i].row_span_sum\n    return sum - major_connected_component.row_span_sum",
    ],
)
```

## `major_raw_contours` decl

```rust
Some(
    [
        "[",
        "RawContour",
        "[]RawContour",
        "~[]RawContour",
    ],
)
```

## `major_raw_contours` defn

```rust
Some(
    [
        "major_connected_component",
        "major_connected_component.raw_contours",
        "major_connected_component.raw_contours",
        "major_connected_component.raw_contours",
    ],
)
```

## `major_raw_contour` decl

```rust
Some(
    [
        "RawContour",
        "~RawContour",
    ],
)
```

## `major_raw_contour` defn

```rust
Some(
    [
        "major_connected_component",
        "major_connected_component.raw_contours",
        "0",
        "major_connected_component.raw_contours[0]",
        "major_connected_component.raw_contours[0]",
        "major_connected_component.raw_contours[0]",
    ],
)
```

## `major_line_segment_sketch` decl

```rust
Some(
    [
        "LineSegmentSketch",
        "~LineSegmentSketch",
    ],
)
```

## `major_line_segment_sketch` defn

```rust
Some(
    [
        "major_raw_contour",
        "major_raw_contour.line_segment_sketch",
        "major_raw_contour.line_segment_sketch",
        "major_raw_contour.line_segment_sketch",
    ],
)
```

## `major_concave_components` decl

```rust
Some(
    [
        "[",
        "ConcaveComponent",
        "[]ConcaveComponent",
        "~[]ConcaveComponent",
    ],
)
```

## `major_concave_components` defn

```rust
Some(
    [
        "major_line_segment_sketch",
        "major_line_segment_sketch.concave_components",
        "major_line_segment_sketch.concave_components",
        "major_line_segment_sketch.concave_components",
    ],
)
```
