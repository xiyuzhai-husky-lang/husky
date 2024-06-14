```rust
Some(
    [
        "let ab = self.displacement()",
        "let ap = self.start.to(pt)",
        "if ab.dot(ap) < 0.0:\n            ap.norm()\n        else:\n            let bp = self.end.to(pt)\n            if ab.dot(bp) > 0.0:\n                bp.norm()\n            else:\n                ab.cross(ap).abs()/ab.norm()",
        "let ab = self.displacement()\n        let ap = self.start.to(pt)\n        if ab.dot(ap) < 0.0:\n            ap.norm()\n        else:\n            let bp = self.end.to(pt)\n            if ab.dot(bp) > 0.0:\n                bp.norm()\n            else:\n                ab.cross(ap).abs()/ab.norm()",
    ],
)
```