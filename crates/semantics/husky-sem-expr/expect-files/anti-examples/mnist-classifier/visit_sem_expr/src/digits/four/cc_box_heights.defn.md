```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y >0.0",
        "dp.y >0.0",
        "require dp.y >0.0",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymin()",
        "0.4",
        "cc.relative_bounding_box.ymin()>0.4",
        "cc.relative_bounding_box.ymin()>0.4",
        "require cc.relative_bounding_box.ymin()>0.4",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymin()",
        "cc.relative_bounding_box.ymin()",
        "let dp = cc.displacement()\n    require dp.y >0.0\n    require cc.relative_bounding_box.ymin()>0.4\n    // dp = cc.displacement()\n    // require dp.y.abs()<7.0\n    cc.relative_bounding_box.ymin()",
    ],
)
```