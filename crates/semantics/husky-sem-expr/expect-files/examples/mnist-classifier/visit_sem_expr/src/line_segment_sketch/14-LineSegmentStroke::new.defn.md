```rust
Some(
    [
        "from",
        "to",
        "from <= to",
        "from <= to",
        "assert from <= to",
        "LineSegmentStroke",
        "ct",
        "ct.points",
        "from",
        "to",
        "1",
        "to + 1",
        "ct.points.cyclic_slice_leashed(from, to + 1)",
        "LineSegmentStroke(ct.points.cyclic_slice_leashed(from, to + 1))",
        "LineSegmentStroke(ct.points.cyclic_slice_leashed(from, to + 1))",
        "assert from <= to\n        LineSegmentStroke(ct.points.cyclic_slice_leashed(from, to + 1))",
    ],
)
```