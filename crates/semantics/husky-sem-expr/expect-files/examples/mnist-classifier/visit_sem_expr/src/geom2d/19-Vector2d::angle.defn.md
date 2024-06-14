```rust
Some(
    [
        "let cos_value = (self.x / self.norm()).min(1.)",
        "if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            (self.y.sgnx() as f32) * cos_value.acos() * 180.0 / 3.1415926",
        "let cos_value = (self.x / self.norm()).min(1.)\n        if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            (self.y.sgnx() as f32) * cos_value.acos() * 180.0 / 3.1415926",
    ],
)
```