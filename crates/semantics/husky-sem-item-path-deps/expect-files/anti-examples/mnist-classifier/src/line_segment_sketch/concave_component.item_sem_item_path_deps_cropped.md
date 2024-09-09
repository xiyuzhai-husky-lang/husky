## `ConcaveComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ],
    ),
)
```

## `find_concave_components`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
            ItemPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`),
        ],
    ),
)
```

## `impl Visualize for ConcaveComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ],
    ),
)
```

## `(ConcaveComponent as Visualize)::visualize`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `impl ConcaveComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ],
    ),
)
```

## `ConcaveComponent::norm`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `ConcaveComponent::rel_norm`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
        ],
    ),
)
```

## `ConcaveComponent::hausdorff_norm`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
            ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
            ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`),
        ],
    ),
)
```

## `ConcaveComponent::angle_change`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::angle_to`),
        ],
    ),
)
```

## `ConcaveComponent::bounding_box`

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

## `ConcaveComponent::relative_bounding_box`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::RelativeBoundingBox`),
            ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
        ],
    ),
)
```

## `ConcaveComponent::line_segment`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
        ],
    ),
)
```

## `ConcaveComponent::start`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
        ],
    ),
)
```

## `ConcaveComponent::end`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Point2d`),
        ],
    ),
)
```

## `ConcaveComponent::displacement`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
            ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`),
        ],
    ),
)
```

## `ConcaveComponent::start_tangent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
        ],
    ),
)
```

## `ConcaveComponent::end_tangent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::geom2d::Vector2d`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
        ],
    ),
)
```
