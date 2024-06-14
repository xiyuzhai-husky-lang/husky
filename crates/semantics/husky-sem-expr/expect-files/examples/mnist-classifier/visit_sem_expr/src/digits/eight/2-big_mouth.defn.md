```rust
Some(
    [
        "if cc.relative_bounding_box.ymax()>0.5:\n        require cc.strokes.first()!.start.x > cc.strokes.first()!.end.x",
        "cc.relative_bounding_box.ymax()",
        "if cc.relative_bounding_box.ymax()>0.5:\n        require cc.strokes.first()!.start.x > cc.strokes.first()!.end.x\n    cc.relative_bounding_box.ymax()",
    ],
)
```