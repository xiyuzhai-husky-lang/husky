```rust
Some(
    [
        "let mut concave_components: []ConcaveComponent = []",
        "let L = line_segment_sketch.strokes.ilen()",
        "let mut start = 0",
        "let mut end = 1",
        "while start > -L and !is_convex(line_segment_sketch, start):\n        start--",
        "let ccv_start = start",
        "while start < ccv_start + L:\n        while end <= start+L and !is_convex(line_segment_sketch, end):\n            end++\n        if end > start + 1:\n            concave_components.push(ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))\n            )\n        start = end\n        end = start + 1",
        "return concave_components",
        "let mut concave_components: []ConcaveComponent = []\n    let L = line_segment_sketch.strokes.ilen()\n    let mut start = 0\n    let mut end = 1\n    while start > -L and !is_convex(line_segment_sketch, start):\n        start--\n    let ccv_start = start\n    while start < ccv_start + L:\n        while end <= start+L and !is_convex(line_segment_sketch, end):\n            end++\n        if end > start + 1:\n            concave_components.push(ConcaveComponent(\n                line_segment_sketch,\n                line_segment_sketch.strokes.cyclic_slice_leashed(start, end))\n            )\n        start = end\n        end = start + 1\n    return concave_components",
    ],
)
```