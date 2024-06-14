```rust
Some(
    [
        "let mut i0 = 0",
        "let mut max_row_span_sum = 0.0",
        "for i < connected_components.ilen():\n        let row_span_sum = connected_components[i].row_span_sum\n        if row_span_sum > max_row_span_sum:\n            max_row_span_sum = row_span_sum\n            i0 = i",
        "return connected_components[i0]",
        "let mut i0 = 0\n    let mut max_row_span_sum = 0.0\n    for i < connected_components.ilen():\n        let row_span_sum = connected_components[i].row_span_sum\n        if row_span_sum > max_row_span_sum:\n            max_row_span_sum = row_span_sum\n            i0 = i\n    return connected_components[i0]",
    ],
)
```