```rust
Some(
    [
        "cc",
        "cc.angle_change",
        "0.0",
        "cc.angle_change + 0.0",
        "140.0",
        "-140.0",
        "cc.angle_change + 0.0 < -140.0",
        "cc.angle_change + 0.0 < -140.0",
        "require cc.angle_change + 0.0 < -140.0",
        "cc",
        "cc.angle_change",
        "-cc.angle_change",
        "0.0",
        "-cc.angle_change + 0.0",
        "-cc.angle_change + 0.0",
        "require cc.angle_change + 0.0 < -140.0\n    -cc.angle_change + 0.0",
    ],
)
```