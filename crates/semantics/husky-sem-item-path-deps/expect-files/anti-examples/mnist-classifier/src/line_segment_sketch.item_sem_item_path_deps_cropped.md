## `concave_component`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `convex_component`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `convexity`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `line_segment`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `LineSegmentStroke`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
        ],
    ),
)
```

## `LineSegmentSketch`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ],
    ),
)
```

## `go_right`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
        ],
    ),
)
```

## `go_left`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
        ],
    ),
)
```

## `extend_end`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
            ItemPath(`mnist_classifier::line_segment_sketch::go_right`),
            ItemPath(`mnist_classifier::line_segment_sketch::go_left`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
        ],
    ),
)
```

## `extend_start`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
            ItemPath(`mnist_classifier::line_segment_sketch::go_right`),
            ItemPath(`mnist_classifier::line_segment_sketch::go_left`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`),
        ],
    ),
)
```

## `find_line_segments`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
            ItemPath(`mnist_classifier::line_segment_sketch::extend_end`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
            ItemPath(`mnist_classifier::line_segment_sketch::extend_start`),
            ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
        ],
    ),
)
```

## `impl Visualize for LineSegmentStroke`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ],
    ),
)
```

## `(LineSegmentStroke as Visualize)::visualize`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `impl LineSegmentStroke`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ],
    ),
)
```

## `LineSegmentStroke::new`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ],
    ),
)
```

## `LineSegmentStroke::displacement`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
        ],
    ),
)
```

## `impl Visualize for LineSegmentSketch`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
        ],
    ),
)
```

## `(LineSegmentSketch as Visualize)::visualize`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `impl LineSegmentSketch`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
        ],
    ),
)
```

## `LineSegmentSketch::concave_components`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`),
        ],
    ),
)
```

## `LineSegmentSketch::bounding_box`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::BoundingBox`),
            ItemPath(`mnist_classifier::geom2d::ClosedRange`),
        ],
    ),
)
```

## `LineSegmentSketch::new`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
            ItemPath(`mnist_classifier::line_segment_sketch::find_line_segments`),
        ],
    ),
)
```
