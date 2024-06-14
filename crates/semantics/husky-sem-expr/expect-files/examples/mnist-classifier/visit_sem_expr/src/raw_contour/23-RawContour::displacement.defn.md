```rust
Some(
    [
        "let N = self.points.ilen()",
        "let ct_start = self.points[start%N]",
        "let ct_end = self.points[end%N]",
        "ct_start.to(ct_end)",
        "let N = self.points.ilen()\n        let ct_start = self.points[start%N]\n        let ct_end = self.points[end%N]\n        ct_start.to(ct_end)",
    ],
)
```