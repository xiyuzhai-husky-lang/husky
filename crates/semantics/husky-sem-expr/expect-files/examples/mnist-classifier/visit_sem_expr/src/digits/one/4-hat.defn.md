```rust
Some(
    [
        "let dp = cc.displacement()",
        "require dp.y < 0.0",
        "require dp.x < 0.0",
        "-dp.y-dp.x",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n    require dp.x < 0.0\n    -dp.y-dp.x",
    ],
)
```