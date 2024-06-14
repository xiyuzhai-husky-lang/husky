```rust
Some(
    [
        "let mut contour_len = 0.0",
        "for 0 < i < self.points.ilen():\n            let a = self.points[i-1]\n            let b = self.points[i]\n            contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()",
        "let a = self.points[self.points.ilen() - 1]",
        "let b = self.points[0]",
        "contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()",
        "return contour_len",
        "let mut contour_len = 0.0\n        for 0 < i < self.points.ilen():\n            let a = self.points[i-1]\n            let b = self.points[i]\n            contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()\n        let a = self.points[self.points.ilen() - 1]\n        let b = self.points[0]\n        contour_len += (a.x - b.x).abs() + (a.y - b.y).abs()\n        return contour_len",
    ],
)
```