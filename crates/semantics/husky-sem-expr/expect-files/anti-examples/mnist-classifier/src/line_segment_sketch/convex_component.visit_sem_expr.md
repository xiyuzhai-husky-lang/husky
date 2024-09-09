## `ConvexComponent` decl

```rust
Some(
    [
        "LineSegmentSketch",
        "~LineSegmentSketch",
        "CyclicSlice",
        "LineSegmentStroke",
        "CyclicSlice LineSegmentStroke",
        "~CyclicSlice LineSegmentStroke",
    ],
)
```

## `ConvexComponent` defn

```rust
None
```

## `impl Visualize for ConvexComponent` decl

```rust
Some(
    [
        "Visualize",
        "ConvexComponent",
    ],
)
```

## `impl Visualize for ConvexComponent` defn

```rust
None
```

## `(ConvexComponent as Visualize)::visualize` decl

```rust
Some(
    [
        "Visual",
    ],
)
```

## `(ConvexComponent as Visualize)::visualize` defn

```rust
Some(
    [
        "self",
        "self.line_segments",
        "self.line_segments.visualize()",
        "self.line_segments.visualize()",
        "self.line_segments.visualize()",
    ],
)
```
