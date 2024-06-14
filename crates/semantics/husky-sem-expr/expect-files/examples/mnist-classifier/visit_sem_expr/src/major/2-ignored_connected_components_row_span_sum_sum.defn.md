```rust
Some(
    [
        "let mut sum = 0.0",
        "for i < connected_components.ilen():\n        sum += connected_components[i].row_span_sum",
        "return sum - major_connected_component.row_span_sum",
        "let mut sum = 0.0\n    for i < connected_components.ilen():\n        sum += connected_components[i].row_span_sum\n    return sum - major_connected_component.row_span_sum",
    ],
)
```