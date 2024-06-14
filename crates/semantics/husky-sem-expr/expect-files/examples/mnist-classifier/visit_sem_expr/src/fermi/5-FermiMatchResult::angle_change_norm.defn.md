```rust
Some(
    [
        "let mut norm: f32 = 0.0",
        "for i < self.others.ilen():\n            norm = norm.max(self.others[i].angle_change.abs())",
        "return norm",
        "let mut norm: f32 = 0.0\n        for i < self.others.ilen():\n            norm = norm.max(self.others[i].angle_change.abs())\n        return norm",
    ],
)
```