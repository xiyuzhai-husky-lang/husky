```rust
Some(
    [
        "let self_norm = self.norm()",
        "assert self_norm > 0.0",
        "let other_norm = other.norm()",
        "assert other_norm > 0.0",
        "let cos_value = (self.dot(other) / (self_norm * other_norm)).min(1.)",
        "if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            let arc_angle = (self.rotation_direction_to(other) as f32) * cos_value.acos()\n            arc_angle * 180.0 / 3.1415926",
        "let self_norm = self.norm()\n        assert self_norm > 0.0\n        let other_norm = other.norm()\n        assert other_norm > 0.0\n        let cos_value = (self.dot(other) / (self_norm * other_norm)).min(1.)\n        if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            let arc_angle = (self.rotation_direction_to(other) as f32) * cos_value.acos()\n            arc_angle * 180.0 / 3.1415926",
    ],
)
```