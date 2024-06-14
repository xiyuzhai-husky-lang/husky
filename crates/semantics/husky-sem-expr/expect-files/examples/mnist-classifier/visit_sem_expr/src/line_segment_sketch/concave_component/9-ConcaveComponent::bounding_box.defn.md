```rust
Some(
    [
        "let start_point = self.strokes.first()!.start",
        "let mut xmin = start_point.x",
        "let mut xmax = start_point.x",
        "let mut ymin = start_point.y",
        "let mut ymax = start_point.y",
        "for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)",
        "return BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
        "let start_point = self.strokes.first()!.start\n        let mut xmin = start_point.x\n        let mut xmax = start_point.x\n        let mut ymin = start_point.y\n        let mut ymax = start_point.y\n        for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)\n        return BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
    ],
)
```