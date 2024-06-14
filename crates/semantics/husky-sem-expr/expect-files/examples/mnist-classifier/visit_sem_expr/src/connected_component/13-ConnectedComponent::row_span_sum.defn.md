```rust
Some(
    [
        "let mut row_span_sum = 0",
        "for 0 < i < 29:\n            row_span_sum += self.mask[i].span()",
        "return row_span_sum as f32",
        "let mut row_span_sum = 0\n        for 0 < i < 29:\n            row_span_sum += self.mask[i].span()\n        return row_span_sum as f32",
    ],
)
```