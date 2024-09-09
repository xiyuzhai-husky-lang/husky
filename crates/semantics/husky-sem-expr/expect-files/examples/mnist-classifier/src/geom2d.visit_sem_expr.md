## `Point2d` decl

```rust
Some(
    [
        "f32",
        "f32",
    ],
)
```

## `Point2d` defn

```rust
None
```

## ``Point2d`::#derive` decl

```rust
Some(
    [
        "Debug",
        "Clone",
        "Visualize",
    ],
)
```

## ``Point2d`::#derive` defn

```rust
None
```

## `RelativePoint2d` decl

```rust
Some(
    [
        "f32",
        "f32",
    ],
)
```

## `RelativePoint2d` defn

```rust
None
```

## `Vector2d` decl

```rust
Some(
    [
        "f32",
        "f32",
    ],
)
```

## `Vector2d` defn

```rust
None
```

## `ClosedRange` decl

```rust
Some(
    [
        "f32",
        "f32",
    ],
)
```

## `ClosedRange` defn

```rust
None
```

## `BoundingBox` decl

```rust
Some(
    [
        "ClosedRange",
        "ClosedRange",
    ],
)
```

## `BoundingBox` defn

```rust
None
```

## `RelativeBoundingBox` decl

```rust
Some(
    [
        "ClosedRange",
        "ClosedRange",
    ],
)
```

## `RelativeBoundingBox` defn

```rust
None
```

## `impl Point2d` decl

```rust
Some(
    [
        "Point2d",
    ],
)
```

## `impl Point2d` defn

```rust
None
```

## `Point2d::from_i_shift28` decl

```rust
Some(
    [
        "i32",
        "i32",
        "Point2d",
    ],
)
```

## `Point2d::from_i_shift28` defn

```rust
Some(
    [
        "Point2d",
        "29",
        "shift",
        "29 - shift",
        "(29 - shift)",
        "f32",
        "(29 - shift) as f32",
        "29",
        "i",
        "29 - i",
        "(29 - i)",
        "f32",
        "(29 - i) as f32",
        "Point2d((29 - shift) as f32, (29 - i) as f32)",
        "Point2d((29 - shift) as f32, (29 - i) as f32)",
        "Point2d((29 - shift) as f32, (29 - i) as f32)",
    ],
)
```

## `Point2d::vector` decl

```rust
Some(
    [
        "Vector2d",
    ],
)
```

## `Point2d::vector` defn

```rust
Some(
    [
        "Vector2d",
        "self",
        "self.x",
        "self",
        "self.y",
        "Vector2d(self.x, self.y)",
        "Vector2d(self.x, self.y)",
        "Vector2d(self.x, self.y)",
    ],
)
```

## `Point2d::to` decl

```rust
Some(
    [
        "Point2d",
        "Vector2d",
    ],
)
```

## `Point2d::to` defn

```rust
Some(
    [
        "Vector2d",
        "other",
        "other.x",
        "self",
        "self.x",
        "other.x - self.x",
        "other",
        "other.y",
        "self",
        "self.y",
        "other.y - self.y",
        "Vector2d(other.x - self.x, other.y - self.y)",
        "Vector2d(other.x - self.x, other.y - self.y)",
        "Vector2d(other.x - self.x, other.y - self.y)",
    ],
)
```

## `Point2d::norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `Point2d::norm` defn

```rust
Some(
    [
        "self",
        "self.x",
        "self",
        "self.x",
        "self.x * self.x",
        "self",
        "self.y",
        "self",
        "self.y",
        "self.y * self.y",
        "self.x * self.x + self.y * self.y",
        "(self.x * self.x + self.y * self.y)",
        "(self.x * self.x + self.y * self.y).sqrt()",
        "(self.x * self.x + self.y * self.y).sqrt()",
        "(self.x * self.x + self.y * self.y).sqrt()",
    ],
)
```

## `Point2d::dist` decl

```rust
Some(
    [
        "Point2d",
        "f32",
    ],
)
```

## `Point2d::dist` defn

```rust
Some(
    [
        "self",
        "other",
        "self.to(other)",
        "self.to(other).norm()",
        "self.to(other).norm()",
        "self.to(other).norm()",
    ],
)
```

## `impl Vector2d` decl

```rust
Some(
    [
        "Vector2d",
    ],
)
```

## `impl Vector2d` defn

```rust
None
```

## `Vector2d::point` decl

```rust
Some(
    [
        "Point2d",
    ],
)
```

## `Vector2d::point` defn

```rust
Some(
    [
        "Point2d",
        "self",
        "self.x",
        "self",
        "self.y",
        "Point2d(self.x, self.y)",
        "Point2d(self.x, self.y)",
        "Point2d(self.x, self.y)",
    ],
)
```

## `Vector2d::to` decl

```rust
Some(
    [
        "Vector2d",
        "Vector2d",
    ],
)
```

## `Vector2d::to` defn

```rust
Some(
    [
        "Vector2d",
        "other",
        "other.x",
        "self",
        "self.x",
        "other.x - self.x",
        "other",
        "other.y",
        "self",
        "self.y",
        "other.y - self.y",
        "Vector2d(other.x - self.x, other.y - self.y)",
        "Vector2d(other.x - self.x, other.y - self.y)",
        "Vector2d(other.x - self.x, other.y - self.y)",
    ],
)
```

## `Vector2d::norm` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `Vector2d::norm` defn

```rust
Some(
    [
        "self",
        "self.x",
        "self",
        "self.x",
        "self.x * self.x",
        "self",
        "self.y",
        "self",
        "self.y",
        "self.y * self.y",
        "self.x * self.x + self.y * self.y",
        "(self.x * self.x + self.y * self.y)",
        "(self.x * self.x + self.y * self.y).sqrt()",
        "(self.x * self.x + self.y * self.y).sqrt()",
        "(self.x * self.x + self.y * self.y).sqrt()",
    ],
)
```

## `Vector2d::dot` decl

```rust
Some(
    [
        "Vector2d",
        "f32",
    ],
)
```

## `Vector2d::dot` defn

```rust
Some(
    [
        "self",
        "self.x",
        "other",
        "other.x",
        "self.x * other.x",
        "self",
        "self.y",
        "other",
        "other.y",
        "self.y * other.y",
        "self.x * other.x + self.y * other.y",
        "self.x * other.x + self.y * other.y",
        "self.x * other.x + self.y * other.y",
    ],
)
```

## `Vector2d::cross` decl

```rust
Some(
    [
        "Vector2d",
        "f32",
    ],
)
```

## `Vector2d::cross` defn

```rust
Some(
    [
        "self",
        "self.x",
        "other",
        "other.y",
        "self.x * other.y",
        "self",
        "self.y",
        "other",
        "other.x",
        "self.y * other.x",
        "self.x * other.y - self.y * other.x",
        "self.x * other.y - self.y * other.x",
        "self.x * other.y - self.y * other.x",
    ],
)
```

## `Vector2d::angle` decl

```rust
Some(
    [
        "bool",
        "f32",
    ],
)
```

## `Vector2d::angle` defn

```rust
Some(
    [
        "self",
        "self.x",
        "self",
        "self.norm()",
        "self.x / self.norm()",
        "(self.x / self.norm())",
        "1.",
        "(self.x / self.norm()).min(1.)",
        "let cos_value = (self.x / self.norm()).min(1.)",
        "cos_value",
        "1.0",
        "cos_value + 1.0",
        "0.001",
        "cos_value + 1.0 < 0.001",
        "cos_value + 1.0 < 0.001",
        "is_branch_cut_positive",
        "is_branch_cut_positive",
        "180.0",
        "180.0",
        "180.0",
        "-180.0",
        "-180.0",
        "if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0",
        "self",
        "self.y",
        "self.y.sgnx()",
        "f32",
        "self.y.sgnx() as f32",
        "(self.y.sgnx() as f32)",
        "cos_value",
        "cos_value.acos()",
        "(self.y.sgnx() as f32) * cos_value.acos()",
        "180.0",
        "(self.y.sgnx() as f32) * cos_value.acos() * 180.0",
        "3.1415926",
        "(self.y.sgnx() as f32) * cos_value.acos() * 180.0 / 3.1415926",
        "(self.y.sgnx() as f32) * cos_value.acos() * 180.0 / 3.1415926",
        "if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            (self.y.sgnx() as f32) * cos_value.acos() * 180.0 / 3.1415926",
        "let cos_value = (self.x / self.norm()).min(1.)\n        if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            (self.y.sgnx() as f32) * cos_value.acos() * 180.0 / 3.1415926",
    ],
)
```

## `Vector2d::rotation_direction_to` decl

```rust
Some(
    [
        "Vector2d",
        "i32",
    ],
)
```

## `Vector2d::rotation_direction_to` defn

```rust
Some(
    [
        "self",
        "other",
        "self.cross(other)",
        "self.cross(other).sgnx()",
        "self.cross(other).sgnx()",
        "self.cross(other).sgnx()",
    ],
)
```

## `Vector2d::angle_to` decl

```rust
Some(
    [
        "Vector2d",
        "bool",
        "f32",
    ],
)
```

## `Vector2d::angle_to` defn

```rust
Some(
    [
        "self",
        "self.norm()",
        "let self_norm = self.norm()",
        "self_norm",
        "0.0",
        "self_norm > 0.0",
        "self_norm > 0.0",
        "assert self_norm > 0.0",
        "other",
        "other.norm()",
        "let other_norm = other.norm()",
        "other_norm",
        "0.0",
        "other_norm > 0.0",
        "other_norm > 0.0",
        "assert other_norm > 0.0",
        "self",
        "other",
        "self.dot(other)",
        "self_norm",
        "other_norm",
        "self_norm * other_norm",
        "(self_norm * other_norm)",
        "self.dot(other) / (self_norm * other_norm)",
        "(self.dot(other) / (self_norm * other_norm))",
        "1.",
        "(self.dot(other) / (self_norm * other_norm)).min(1.)",
        "let cos_value = (self.dot(other) / (self_norm * other_norm)).min(1.)",
        "cos_value",
        "1.0",
        "cos_value + 1.0",
        "0.001",
        "cos_value + 1.0 < 0.001",
        "cos_value + 1.0 < 0.001",
        "is_branch_cut_positive",
        "is_branch_cut_positive",
        "180.0",
        "180.0",
        "180.0",
        "-180.0",
        "-180.0",
        "if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0",
        "self",
        "other",
        "self.rotation_direction_to(other)",
        "f32",
        "self.rotation_direction_to(other) as f32",
        "(self.rotation_direction_to(other) as f32)",
        "cos_value",
        "cos_value.acos()",
        "(self.rotation_direction_to(other) as f32) * cos_value.acos()",
        "let arc_angle = (self.rotation_direction_to(other) as f32) * cos_value.acos()",
        "arc_angle",
        "180.0",
        "arc_angle * 180.0",
        "3.1415926",
        "arc_angle * 180.0 / 3.1415926",
        "arc_angle * 180.0 / 3.1415926",
        "if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            let arc_angle = (self.rotation_direction_to(other) as f32) * cos_value.acos()\n            arc_angle * 180.0 / 3.1415926",
        "let self_norm = self.norm()\n        assert self_norm > 0.0\n        let other_norm = other.norm()\n        assert other_norm > 0.0\n        let cos_value = (self.dot(other) / (self_norm * other_norm)).min(1.)\n        if cos_value + 1.0 < 0.001:\n            if is_branch_cut_positive:\n                180.0\n            else:\n                -180.0\n        else:\n            let arc_angle = (self.rotation_direction_to(other) as f32) * cos_value.acos()\n            arc_angle * 180.0 / 3.1415926",
    ],
)
```

## `impl ClosedRange` decl

```rust
Some(
    [
        "ClosedRange",
    ],
)
```

## `impl ClosedRange` defn

```rust
None
```

## `ClosedRange::relative_range` decl

```rust
Some(
    [
        "ClosedRange",
        "ClosedRange",
    ],
)
```

## `ClosedRange::relative_range` defn

```rust
Some(
    [
        "self",
        "self.max",
        "self",
        "self.min",
        "self.max > self.min",
        "self.max > self.min",
        "assert self.max > self.min",
        "self",
        "self.max",
        "self",
        "self.min",
        "self.max - self.min",
        "let span = self.max - self.min",
        "other",
        "other.min",
        "self",
        "self.min",
        "other.min - self.min",
        "(other.min - self.min)",
        "span",
        "(other.min - self.min) / span",
        "let rel_min = (other.min - self.min) / span",
        "other",
        "other.max",
        "self",
        "self.min",
        "other.max - self.min",
        "(other.max - self.min)",
        "span",
        "(other.max - self.min) / span",
        "let rel_max = (other.max - self.min) / span",
        "ClosedRange",
        "rel_min",
        "rel_max",
        "ClosedRange(\n            rel_min,\n            rel_max,\n        )",
        "ClosedRange(\n            rel_min,\n            rel_max,\n        )",
        "assert self.max > self.min\n        let span = self.max - self.min\n        let rel_min = (other.min - self.min) / span\n        let rel_max = (other.max - self.min) / span\n        ClosedRange(\n            rel_min,\n            rel_max,\n        )",
    ],
)
```

## `ClosedRange::relative_point` decl

```rust
Some(
    [
        "f32",
        "f32",
    ],
)
```

## `ClosedRange::relative_point` defn

```rust
Some(
    [
        "self",
        "self.max",
        "self",
        "self.min",
        "self.max - self.min",
        "let span = self.max - self.min",
        "v",
        "self",
        "self.min",
        "v - self.min",
        "(v - self.min)",
        "span",
        "(v - self.min) / span",
        "(v - self.min) / span",
        "let span = self.max - self.min\n        (v - self.min) / span",
    ],
)
```

## `impl BoundingBox` decl

```rust
Some(
    [
        "BoundingBox",
    ],
)
```

## `impl BoundingBox` defn

```rust
None
```

## `BoundingBox::relative_bounding_box` decl

```rust
Some(
    [
        "BoundingBox",
        "RelativeBoundingBox",
    ],
)
```

## `BoundingBox::relative_bounding_box` defn

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

## `BoundingBox::relative_point` decl

```rust
Some(
    [
        "Point2d",
        "RelativePoint2d",
    ],
)
```

## `BoundingBox::relative_point` defn

```rust
Some(
    [
        "RelativePoint2d",
        "self",
        "self.xrange",
        "point",
        "point.x",
        "self.xrange.relative_point(point.x)",
        "self",
        "self.yrange",
        "point",
        "point.x",
        "self.yrange.relative_point(point.x)",
        "RelativePoint2d(\n            self.xrange.relative_point(point.x),\n            self.yrange.relative_point(point.x),\n        )",
        "RelativePoint2d(\n            self.xrange.relative_point(point.x),\n            self.yrange.relative_point(point.x),\n        )",
        "RelativePoint2d(\n            self.xrange.relative_point(point.x),\n            self.yrange.relative_point(point.x),\n        )",
    ],
)
```

## `BoundingBox::xmin` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `BoundingBox::xmin` defn

```rust
Some(
    [
        "self",
        "self.xrange",
        "self.xrange.min",
        "self.xrange.min",
        "self.xrange.min",
    ],
)
```

## `BoundingBox::xmax` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `BoundingBox::xmax` defn

```rust
Some(
    [
        "self",
        "self.xrange",
        "self.xrange.max",
        "self.xrange.max",
        "self.xrange.max",
    ],
)
```

## `BoundingBox::ymin` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `BoundingBox::ymin` defn

```rust
Some(
    [
        "self",
        "self.yrange",
        "self.yrange.min",
        "self.yrange.min",
        "self.yrange.min",
    ],
)
```

## `BoundingBox::ymax` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `BoundingBox::ymax` defn

```rust
Some(
    [
        "self",
        "self.yrange",
        "self.yrange.max",
        "self.yrange.max",
        "self.yrange.max",
    ],
)
```

## `impl RelativeBoundingBox` decl

```rust
Some(
    [
        "RelativeBoundingBox",
    ],
)
```

## `impl RelativeBoundingBox` defn

```rust
None
```

## `RelativeBoundingBox::xmin` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `RelativeBoundingBox::xmin` defn

```rust
Some(
    [
        "self",
        "self.xrange",
        "self.xrange.min",
        "self.xrange.min",
        "self.xrange.min",
    ],
)
```

## `RelativeBoundingBox::xmax` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `RelativeBoundingBox::xmax` defn

```rust
Some(
    [
        "self",
        "self.xrange",
        "self.xrange.max",
        "self.xrange.max",
        "self.xrange.max",
    ],
)
```

## `RelativeBoundingBox::ymin` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `RelativeBoundingBox::ymin` defn

```rust
Some(
    [
        "self",
        "self.yrange",
        "self.yrange.min",
        "self.yrange.min",
        "self.yrange.min",
    ],
)
```

## `RelativeBoundingBox::ymax` decl

```rust
Some(
    [
        "f32",
    ],
)
```

## `RelativeBoundingBox::ymax` defn

```rust
Some(
    [
        "self",
        "self.yrange",
        "self.yrange.max",
        "self.yrange.max",
        "self.yrange.max",
    ],
)
```
