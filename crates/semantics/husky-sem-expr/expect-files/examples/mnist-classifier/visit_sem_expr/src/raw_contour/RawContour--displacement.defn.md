```rust
Some(
    [
        "self",
        "self.points",
        "self.points.ilen()",
        "let N = self.points.ilen()",
        "self",
        "self.points",
        "start",
        "N",
        "start%N",
        "self.points[start%N]",
        "let ct_start = self.points[start%N]",
        "self",
        "self.points",
        "end",
        "N",
        "end%N",
        "self.points[end%N]",
        "let ct_end = self.points[end%N]",
        "ct_start",
        "ct_end",
        "ct_start.to(ct_end)",
        "ct_start.to(ct_end)",
        "let N = self.points.ilen()\n        let ct_start = self.points[start%N]\n        let ct_end = self.points[end%N]\n        ct_start.to(ct_end)",
    ],
)
```