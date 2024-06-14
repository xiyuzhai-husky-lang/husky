```rust
Some(
    [
        "let dp = cc.displacement()",
        "require dp.y < 0.0",
        "require cc.relative_bounding_box.ymax() > 0.6",
        "cc.end().y",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n    require cc.relative_bounding_box.ymax() > 0.6\n    cc.end().y",
    ],
)
```