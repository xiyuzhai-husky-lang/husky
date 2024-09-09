## `ConcaveComponent` decl

```rust
Some(
    [
        "LineSegmentSketch",
        "~LineSegmentSketch",
        "CyclicSlice",
        "LineSegmentStroke",
        "CyclicSlice LineSegmentStroke",
        "~CyclicSlice LineSegmentStroke",
    ],
)
```

## `ConcaveComponent` defn

```rust
None
```

## `find_concave_components` decl

```rust
Some(
    [
        "LineSegmentSketch",
        "~LineSegmentSketch",
        "[",
        "ConcaveComponent",
        "[]ConcaveComponent",
    ],
)
```

## `find_concave_components` defn

```rust
Some(
    [
        "[]",
        "let mut concave_components: []ConcaveComponent = []",
        "line_segment_sketch",
        "line_segment_sketch.strokes",
        "line_segment_sketch.strokes.ilen()",
        "let L = line_segment_sketch.strokes.ilen()",
        "0",
        "let mut start = 0",
        "1",
        "let mut end = 1",
        "start",
        "L",
        "-L",
        "start > -L",
        "is_convex",
        "line_segment_sketch",
        "start",
        "is_convex(line_segment_sketch, start)",
        "!is_convex(line_segment_sketch, start)",
        "start > -L and !is_convex(line_segment_sketch, start)",
        "start > -L and !is_convex(line_segment_sketch, start)",
        "start",
        "start--",
        "start--",
        "while start > -L and !is_convex(line_segment_sketch, start):\n        start--",
        "start",
        "let ccv_start = start",
        "start",
        "ccv_start",
        "L",
        "ccv_start + L",
        "start < ccv_start + L",
        "start < ccv_start + L",
        "end",
        "start",
        "L",
        "start+L",
        "end <= start+L",
        "is_convex",
        "line_segment_sketch",
        "end",
        "is_convex(line_segment_sketch, end)",
        "!is_convex(line_segment_sketch, end)",
        "end <= start+L and !is_convex(line_segment_sketch, end)",
        "end <= start+L and !is_convex(line_segment_sketch, end)",
        "end",
        "end++",
        "end++",
        "while end <= start+L and !is_convex(line_segment_sketch, end):\n            end++",
        "end",
        "start",
        "1",
        "start + 1",
        "end > start + 1",
        "end > start + 1",
        "concave_components",
        "ConcaveComponent",
        "line_segment_sketch",
        "line_segment_sketch",
        "line_segment_sketch.strokes",
        "start",
        "end",
        "line_segment_sketch.strokes.cyclic_slice_leashed(start, end)",
        "ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))",
        "concave_components.push(ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))\n            )",
        "concave_components.push(ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))\n            )",
        "if end > start + 1:\n            concave_components.push(ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))\n            )",
        "start",
        "end",
        "start = end",
        "start = end",
        "end",
        "start",
        "1",
        "start + 1",
        "end = start + 1",
        "end = start + 1",
        "while start < ccv_start + L:\n        while end <= start+L and !is_convex(line_segment_sketch, end):\n            end++\n        if end > start + 1:\n            concave_components.push(ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))\n            )\n        start = end\n        end = start + 1",
        "concave_components",
        "return concave_components",
        "let mut concave_components: []ConcaveComponent = []\n    let L = line_segment_sketch.strokes.ilen()\n    let mut start = 0\n    let mut end = 1\n    while start > -L and !is_convex(line_segment_sketch, start):\n        start--\n    let ccv_start = start\n    while start < ccv_start + L:\n        while end <= start+L and !is_convex(line_segment_sketch, end):\n            end++\n        if end > start + 1:\n            concave_components.push(ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))\n            )\n        start = end\n        end = start + 1\n    return concave_components",
    ],
)
```

## `impl Visualize for ConcaveComponent` decl

```rust
Some(
    [
        "Visualize",
        "ConcaveComponent",
    ],
)
```

## `impl Visualize for ConcaveComponent` defn

```rust
None
```

## `(ConcaveComponent as Visualize)::visualize` decl

```rust
Some(
    [
        "Visual",
    ],
)
```

## `(ConcaveComponent as Visualize)::visualize` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "self.strokes.visualize()",
        "self.strokes.visualize()",
        "self.strokes.visualize()",
    ],
)
```

## `impl ConcaveComponent` decl

```rust
Some(
    [
        "ConcaveComponent",
    ],
)
```

## `impl ConcaveComponent` defn

```rust
None
```

## `ConcaveComponent::norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConcaveComponent::norm` defn

```rust
Some(
    [
        "self",
        "self.hausdorff_norm",
        "self.hausdorff_norm",
        "self.hausdorff_norm",
    ],
)
```

## `ConcaveComponent::rel_norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConcaveComponent::rel_norm` defn

```rust
Some(
    [
        "self",
        "self.norm",
        "self",
        "self.displacement()",
        "self.displacement().norm()",
        "self.norm / self.displacement().norm()",
        "self.norm / self.displacement().norm()",
        "self.norm / self.displacement().norm()",
    ],
)
```

## `ConcaveComponent::hausdorff_norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConcaveComponent::hausdorff_norm` defn

```rust
Some(
    [
        "0.0",
        "let mut hausdorff_norm = 0.0",
        "self",
        "self.strokes",
        "self.strokes.first()",
        "self.strokes.first()!",
        "self.strokes.first()!.start",
        "let curve_start = self.strokes.first()!.start",
        "self",
        "self.line_segment()",
        "let curve_ls = self.line_segment()",
        "curve_ls",
        "curve_ls.displacement()",
        "curve_ls.displacement().norm()",
        "let dp_norm = curve_ls.displacement().norm()",
        "self",
        "self.strokes",
        "self.strokes.start()",
        "self",
        "self.strokes",
        "self.strokes.end()",
        "i",
        "self",
        "self.strokes",
        "i",
        "self.strokes[i]",
        "self.strokes[i].end",
        "let point = self.strokes[i].end",
        "curve_ls",
        "point",
        "curve_ls.dist_to_point(point)",
        "let point_dist = curve_ls.dist_to_point(point)",
        "point_dist",
        "hausdorff_norm",
        "point_dist > hausdorff_norm",
        "point_dist > hausdorff_norm",
        "hausdorff_norm",
        "point_dist",
        "hausdorff_norm = point_dist",
        "hausdorff_norm = point_dist",
        "if point_dist > hausdorff_norm:\n                hausdorff_norm = point_dist",
        "for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            let point_dist = curve_ls.dist_to_point(point)\n            if point_dist > hausdorff_norm:\n                hausdorff_norm = point_dist",
        "hausdorff_norm",
        "return hausdorff_norm",
        "let mut hausdorff_norm = 0.0\n        let curve_start = self.strokes.first()!.start\n        let curve_ls = self.line_segment()\n        let dp_norm = curve_ls.displacement().norm()\n        // todo: change self to iter().max()\n        for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            let point_dist = curve_ls.dist_to_point(point)\n            if point_dist > hausdorff_norm:\n                hausdorff_norm = point_dist\n        return hausdorff_norm",
    ],
)
```

## `ConcaveComponent::angle_change` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `ConcaveComponent::angle_change` defn

```rust
Some(
    [
        "0.0",
        "let mut angle_change = 0.0",
        "self",
        "self.strokes",
        "self",
        "self.strokes",
        "self.strokes.start()",
        "self.strokes[self.strokes.start()]",
        "self.strokes[self.strokes.start()].displacement()",
        "let mut dp0 = self.strokes[self.strokes.start()].displacement()",
        "self",
        "self.strokes",
        "self.strokes.start()",
        "self",
        "self.strokes",
        "self.strokes.end()",
        "i",
        "self",
        "self.strokes",
        "i",
        "self.strokes[i]",
        "self.strokes[i].displacement()",
        "let dp = self.strokes[i].displacement()",
        "angle_change",
        "dp0",
        "dp",
        "true",
        "dp0.angle_to(dp, true)",
        "angle_change += dp0.angle_to(dp, true)",
        "angle_change += dp0.angle_to(dp, true)",
        "dp0",
        "dp",
        "dp0 = dp",
        "dp0 = dp",
        "for self.strokes.start() < i < self.strokes.end():\n            let dp = self.strokes[i].displacement()\n            angle_change += dp0.angle_to(dp, true)\n            dp0 = dp",
        "angle_change",
        "return angle_change",
        "let mut angle_change = 0.0\n        let mut dp0 = self.strokes[self.strokes.start()].displacement()\n        // todo: change self for .. in ..[1..]\n        for self.strokes.start() < i < self.strokes.end():\n            let dp = self.strokes[i].displacement()\n            angle_change += dp0.angle_to(dp, true)\n            dp0 = dp\n        return angle_change",
    ],
)
```

## `ConcaveComponent::bounding_box` decl

```rust
Some(
    [
        "BoundingBox",
    ],
)
```

## `ConcaveComponent::bounding_box` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "self.strokes.first()",
        "self.strokes.first()!",
        "self.strokes.first()!.start",
        "let start_point = self.strokes.first()!.start",
        "start_point",
        "start_point.x",
        "let mut xmin = start_point.x",
        "start_point",
        "start_point.x",
        "let mut xmax = start_point.x",
        "start_point",
        "start_point.y",
        "let mut ymin = start_point.y",
        "start_point",
        "start_point.y",
        "let mut ymax = start_point.y",
        "self",
        "self.strokes",
        "self.strokes.start()",
        "self",
        "self.strokes",
        "self.strokes.end()",
        "i",
        "self",
        "self.strokes",
        "i",
        "self.strokes[i]",
        "self.strokes[i].end",
        "let point = self.strokes[i].end",
        "xmin",
        "xmin",
        "point",
        "point.x",
        "xmin.min(point.x)",
        "xmin = xmin.min(point.x)",
        "xmin = xmin.min(point.x)",
        "xmax",
        "xmax",
        "point",
        "point.x",
        "xmax.max(point.x)",
        "xmax = xmax.max(point.x)",
        "xmax = xmax.max(point.x)",
        "ymin",
        "ymin",
        "point",
        "point.y",
        "ymin.min(point.y)",
        "ymin = ymin.min(point.y)",
        "ymin = ymin.min(point.y)",
        "ymax",
        "ymax",
        "point",
        "point.y",
        "ymax.max(point.y)",
        "ymax = ymax.max(point.y)",
        "ymax = ymax.max(point.y)",
        "for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)",
        "BoundingBox",
        "ClosedRange",
        "xmin",
        "xmax",
        "ClosedRange(xmin, xmax)",
        "ClosedRange",
        "ymin",
        "ymax",
        "ClosedRange(ymin, ymax)",
        "BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
        "return BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
        "let start_point = self.strokes.first()!.start\n        let mut xmin = start_point.x\n        let mut xmax = start_point.x\n        let mut ymin = start_point.y\n        let mut ymax = start_point.y\n        for self.strokes.start() <= i < self.strokes.end():\n            let point = self.strokes[i].end\n            xmin = xmin.min(point.x)\n            xmax = xmax.max(point.x)\n            ymin = ymin.min(point.y)\n            ymax = ymax.max(point.y)\n        return BoundingBox(\n            ClosedRange(xmin, xmax), \n            ClosedRange(ymin, ymax),\n        )",
    ],
)
```

## `ConcaveComponent::relative_bounding_box` decl

```rust
Some(
    [
        "RelativeBoundingBox",
    ],
)
```

## `ConcaveComponent::relative_bounding_box` defn

```rust
Some(
    [
        "self",
        "self.line_segment_sketch",
        "self.line_segment_sketch.bounding_box",
        "self",
        "self.bounding_box",
        "self.line_segment_sketch.bounding_box.relative_bounding_box(self.bounding_box)",
        "self.line_segment_sketch.bounding_box.relative_bounding_box(self.bounding_box)",
        "self.line_segment_sketch.bounding_box.relative_bounding_box(self.bounding_box)",
    ],
)
```

## `ConcaveComponent::line_segment` decl

```rust
Some(
    [
        "LineSegment",
    ],
)
```

## `ConcaveComponent::line_segment` defn

```rust
Some(
    [
        "LineSegment",
        "self",
        "self.strokes",
        "self.strokes.first()",
        "self.strokes.first()!",
        "self.strokes.first()!.start",
        "self.strokes.first()!.start.clone()",
        "self",
        "self.strokes",
        "self.strokes.last()",
        "self.strokes.last()!",
        "self.strokes.last()!.end",
        "self.strokes.last()!.end.clone()",
        "LineSegment(\n            self.strokes.first()!.start.clone(),\n            self.strokes.last()!.end.clone()\n        )",
        "LineSegment(\n            self.strokes.first()!.start.clone(),\n            self.strokes.last()!.end.clone()\n        )",
        "LineSegment(\n            self.strokes.first()!.start.clone(),\n            self.strokes.last()!.end.clone()\n        )",
    ],
)
```

## `ConcaveComponent::start` decl

```rust
Some(
    [
        "Point2d",
    ],
)
```

## `ConcaveComponent::start` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "self.strokes.first()",
        "self.strokes.first()!",
        "self.strokes.first()!.start",
        "self.strokes.first()!.start.clone()",
        "self.strokes.first()!.start.clone()",
        "self.strokes.first()!.start.clone()",
    ],
)
```

## `ConcaveComponent::end` decl

```rust
Some(
    [
        "Point2d",
    ],
)
```

## `ConcaveComponent::end` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "self.strokes.last()",
        "self.strokes.last()!",
        "self.strokes.last()!.end",
        "self.strokes.last()!.end.clone()",
        "self.strokes.last()!.end.clone()",
        "self.strokes.last()!.end.clone()",
    ],
)
```

## `ConcaveComponent::displacement` decl

```rust
Some(
    [
        "Vector2d",
    ],
)
```

## `ConcaveComponent::displacement` defn

```rust
Some(
    [
        "self",
        "self.line_segment()",
        "self.line_segment().displacement()",
        "self.line_segment().displacement()",
        "self.line_segment().displacement()",
    ],
)
```

## `ConcaveComponent::start_tangent` decl

```rust
Some(
    [
        "Vector2d",
    ],
)
```

## `ConcaveComponent::start_tangent` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "self.strokes.first()",
        "self.strokes.first()!",
        "self.strokes.first()!.displacement()",
        "self.strokes.first()!.displacement()",
        "self.strokes.first()!.displacement()",
    ],
)
```

## `ConcaveComponent::end_tangent` decl

```rust
Some(
    [
        "Vector2d",
    ],
)
```

## `ConcaveComponent::end_tangent` defn

```rust
Some(
    [
        "self",
        "self.strokes",
        "self.strokes.last()",
        "self.strokes.last()!",
        "self.strokes.last()!.displacement()",
        "self.strokes.last()!.displacement()",
        "self.strokes.last()!.displacement()",
    ],
)
```
