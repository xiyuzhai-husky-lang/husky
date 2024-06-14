```rust
Some(
    [
        "let dp = cc.displacement()",
        "require dp.y < 0.0",
        "require cc.relative_bounding_box.ymin() < 0.3",
        "let ang = cc.start_tangent().angle(true)",
        "require ang < 30.0",
        "ang",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n\n    require cc.relative_bounding_box.ymin() < 0.3\n\n    let ang = cc.start_tangent().angle(true)\n    require ang < 30.0\n    ang",
    ],
)
```