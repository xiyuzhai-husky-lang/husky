## `LineSegment` decl

```rust
Some(
    [
        "Point2d",
        "Point2d",
    ],
)
```

## `LineSegment` defn

```rust
None
```

## `impl LineSegment` decl

```rust
Some(
    [
        "LineSegment",
    ],
)
```

## `impl LineSegment` defn

```rust
None
```

## `LineSegment::displacement` decl

```rust
Some(
    [
        "Vector2d",
    ],
)
```

## `LineSegment::displacement` defn

```rust
Some(
    [
        "self",
        "self.start",
        "self",
        "self.end",
        "self.start.to(self.end)",
        "self.start.to(self.end)",
        "self.start.to(self.end)",
    ],
)
```

## `LineSegment::dist_to_point` decl

```rust
Some(
    [
        "Point2d",
        "f32",
    ],
)
```

## `LineSegment::dist_to_point` defn

```rust
Some(
    [
        "self",
        "self.displacement()",
        "let ab = self.displacement()",
        "self",
        "self.start",
        "pt",
        "self.start.to(pt)",
        "let ap = self.start.to(pt)",
        "ab",
        "ap",
        "ab.dot(ap)",
        "0.0",
        "ab.dot(ap) < 0.0",
        "ab.dot(ap) < 0.0",
        "ap",
        "ap.norm()",
        "ap.norm()",
        "self",
        "self.end",
        "pt",
        "self.end.to(pt)",
        "let bp = self.end.to(pt)",
        "ab",
        "bp",
        "ab.dot(bp)",
        "0.0",
        "ab.dot(bp) > 0.0",
        "ab.dot(bp) > 0.0",
        "bp",
        "bp.norm()",
        "bp.norm()",
        "ab",
        "ap",
        "ab.cross(ap)",
        "ab.cross(ap).abs()",
        "ab",
        "ab.norm()",
        "ab.cross(ap).abs()/ab.norm()",
        "ab.cross(ap).abs()/ab.norm()",
        "if ab.dot(bp) > 0.0:\n                bp.norm()\n            else:\n                ab.cross(ap).abs()/ab.norm()",
        "if ab.dot(ap) < 0.0:\n            ap.norm()\n        else:\n            let bp = self.end.to(pt)\n            if ab.dot(bp) > 0.0:\n                bp.norm()\n            else:\n                ab.cross(ap).abs()/ab.norm()",
        "let ab = self.displacement()\n        let ap = self.start.to(pt)\n        if ab.dot(ap) < 0.0:\n            ap.norm()\n        else:\n            let bp = self.end.to(pt)\n            if ab.dot(bp) > 0.0:\n                bp.norm()\n            else:\n                ab.cross(ap).abs()/ab.norm()",
    ],
)
```
