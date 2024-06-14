```rust
Some(
    [
        "let mut row_start = 1",
        "forext row_start < 29:\n            if self.mask[row_start]:\n                break",
        "let mut row_end = row_start + 1",
        "forext row_end < 29:\n            if !self.mask[row_end]:\n                break",
        "let height = row_end - row_start",
        "let half_height = height / 2",
        "let mut upper_mass = 0",
        "for row_start <= i1 < row_start + half_height:\n            upper_mass += self.mask[i1].co()",
        "let mut lower_mass = 0",
        "for row_end > i2 >= row_end - half_height:\n            lower_mass += self.mask[i2].co()",
        "return ConnectedComponentDistribution(\n            row_start,\n            row_end,\n            upper_mass,\n            lower_mass,\n        )",
        "let mut row_start = 1\n        forext row_start < 29:\n            if self.mask[row_start]:\n                break\n        let mut row_end = row_start + 1\n        forext row_end < 29:\n            if !self.mask[row_end]:\n                break\n        let height = row_end - row_start\n        let half_height = height / 2\n        let mut upper_mass = 0\n        for row_start <= i1 < row_start + half_height:\n            upper_mass += self.mask[i1].co()\n        let mut lower_mass = 0\n        for row_end > i2 >= row_end - half_height:\n            lower_mass += self.mask[i2].co()\n        return ConnectedComponentDistribution(\n            row_start,\n            row_end,\n            upper_mass,\n            lower_mass,\n        )",
    ],
)
```