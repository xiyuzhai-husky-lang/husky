```rust
Some(
    [
        "let mut max_hole_ilen = 0",
        "let raw_contours = self.raw_contours",
        "for 0 < i < raw_contours.ilen():\n            let hole_ilen = raw_contours[i].points.ilen()\n            if max_hole_ilen < hole_ilen:\n                max_hole_ilen = hole_ilen",
        "return max_hole_ilen as f32",
        "let mut max_hole_ilen = 0\n        let raw_contours = self.raw_contours\n        for 0 < i < raw_contours.ilen():\n            let hole_ilen = raw_contours[i].points.ilen()\n            if max_hole_ilen < hole_ilen:\n                max_hole_ilen = hole_ilen\n        return max_hole_ilen as f32",
    ],
)
```