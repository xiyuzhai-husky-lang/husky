```rust
Some(
    [
        "let dp = cc.displacement()",
        "if dp.y < -3.0:\n        require (dp.x / dp.y).abs() > 1.4",
        "require cc.relative_bounding_box.ymax() < 0.6",
        "let relative_end = cc.line_segment_sketch.bounding_box.relative_point(cc.end())",
        "require relative_end.x > 0.5",
        "-cc.end().y",
        "let dp = cc.displacement()\n    if dp.y < -3.0:\n        require (dp.x / dp.y).abs() > 1.4\n    require cc.relative_bounding_box.ymax() < 0.6\n    let relative_end = cc.line_segment_sketch.bounding_box.relative_point(cc.end())\n    require relative_end.x > 0.5\n    -cc.end().y",
    ],
)
```