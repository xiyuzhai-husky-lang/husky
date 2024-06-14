```rust
Some(
    [
        "let mut raw_contours = self.raw_contours.collect_leashes()",
        "let mut matches: []?~RawContour = []",
        "raw_contours.pop_with_largest_opt_f32(hole_tmpl)",
        "matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))",
        "matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))",
        "return EffHoles(matches)",
        "let mut raw_contours = self.raw_contours.collect_leashes()\n        let mut matches: []?~RawContour = []\n        // ad hoc, should replace self with pop_first\n        raw_contours.pop_with_largest_opt_f32(hole_tmpl);\n        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))\n        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl))\n        return EffHoles(matches)",
    ],
)
```