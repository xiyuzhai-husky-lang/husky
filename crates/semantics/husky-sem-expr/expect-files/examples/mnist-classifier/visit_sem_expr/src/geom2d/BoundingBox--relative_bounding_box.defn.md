```rust
Some(
    [
        "RelativeBoundingBox",
        "self",
        "self.xrange",
        "other",
        "other.xrange",
        "self.xrange.relative_range(other.xrange)",
        "self",
        "self.yrange",
        "other",
        "other.yrange",
        "self.yrange.relative_range(other.yrange)",
        "RelativeBoundingBox(\n            self.xrange.relative_range(other.xrange),\n            self.yrange.relative_range(other.yrange),\n        )",
        "RelativeBoundingBox(\n            self.xrange.relative_range(other.xrange),\n            self.yrange.relative_range(other.yrange),\n        )",
        "RelativeBoundingBox(\n            self.xrange.relative_range(other.xrange),\n            self.yrange.relative_range(other.yrange),\n        )",
    ],
)
```