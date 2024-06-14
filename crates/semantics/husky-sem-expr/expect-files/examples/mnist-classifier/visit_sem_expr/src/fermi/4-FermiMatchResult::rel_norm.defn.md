```rust
Some(
    [
        "let mut norm: f32 = 0.0",
        "for i < self.others.ilen():\n            norm = norm.max(self.others[i].rel_norm)",
        "return norm",
        "let mut norm: f32 = 0.0\n        for i < self.others.ilen():\n            norm = norm.max(self.others[i].rel_norm)\n        return norm",
    ],
)
```