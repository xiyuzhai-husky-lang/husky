```rust
Some(
    [
        "let dp = cc.displacement()",
        "require dp.y >0.0",
        "require cc.relative_bounding_box.ymin()>0.4",
        "cc.relative_bounding_box.ymin()",
        "let dp = cc.displacement()\n    require dp.y >0.0\n    require cc.relative_bounding_box.ymin()>0.4\n    // dp = cc.displacement()\n    // require dp.y.abs()<7.0\n    cc.relative_bounding_box.ymin()",
    ],
)
```