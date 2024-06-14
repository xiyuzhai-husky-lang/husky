```rust
Some(
    [
        "let mut hausdorff_norm = 0.0",
        "let curve_start = self.strokes.first()!.start",
        "let curve_ls = self.line_segment()",
        "let dp_norm = curve_ls.displacement().norm()",
        "for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            let point_dist = curve_ls.dist_to_point(point)\n            if point_dist > hausdorff_norm:\n                hausdorff_norm = point_dist",
        "return hausdorff_norm",
        "let mut hausdorff_norm = 0.0\n        let curve_start = self.strokes.first()!.start\n        let curve_ls = self.line_segment()\n        let dp_norm = curve_ls.displacement().norm()\n        // todo: change self to iter().max()\n        for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            let point_dist = curve_ls.dist_to_point(point)\n            if point_dist > hausdorff_norm:\n                hausdorff_norm = point_dist\n        return hausdorff_norm",
    ],
)
```