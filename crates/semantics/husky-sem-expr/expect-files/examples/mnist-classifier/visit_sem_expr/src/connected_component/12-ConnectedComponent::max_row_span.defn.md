```rust
Some(
    [
        "let mut max_row: i32 = 0",
        "for 0 < i < 29:\n            max_row = max_row.max(self.mask[i].span())",
        "return max_row as f32",
        "let mut max_row: i32 = 0\n        for 0 < i < 29:\n            max_row = max_row.max(self.mask[i].span())\n        return max_row as f32",
    ],
)
```