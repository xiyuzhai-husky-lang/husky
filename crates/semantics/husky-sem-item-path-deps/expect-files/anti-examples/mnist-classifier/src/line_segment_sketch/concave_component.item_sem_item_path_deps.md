## `ConcaveComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
            ItemPath(`core::slice::CyclicSlice`),
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
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`),
            ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
            ItemPath(`core::vec::Vec(0)::push`),
        ],
    ),
)
```

## `impl Visualize for ConcaveComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visualize`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
        ],
    ),
)
```

## `(ConcaveComponent as Visualize)::visualize`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visual`),
            ItemPath(`<#derive _ as core::visual::Visualize(0)>::visualize`),
        ],
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
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `ConcaveComponent::rel_norm`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
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
            ItemPath(`core::num::f32`),
            ItemPath(`core::slice::CyclicSlice(0)::first`),
            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
            ItemPath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
            ItemPath(`core::slice::CyclicSlice(0)::start`),
            ItemPath(`core::slice::CyclicSlice(0)::end`),
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
            ItemPath(`core::num::f32`),
            ItemPath(`core::slice::CyclicSlice(0)::start`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
            ItemPath(`core::slice::CyclicSlice(0)::end`),
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
            ItemPath(`core::slice::CyclicSlice(0)::first`),
            ItemPath(`core::slice::CyclicSlice(0)::start`),
            ItemPath(`core::slice::CyclicSlice(0)::end`),
            ItemPath(`core::num::f32(0)::min`),
            ItemPath(`core::num::f32(0)::max`),
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
            ItemPath(`core::slice::CyclicSlice(0)::first`),
            ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
            ItemPath(`core::slice::CyclicSlice(0)::last`),
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
            ItemPath(`core::slice::CyclicSlice(0)::first`),
            ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
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
            ItemPath(`core::slice::CyclicSlice(0)::last`),
            ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
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
            ItemPath(`core::slice::CyclicSlice(0)::first`),
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
            ItemPath(`core::slice::CyclicSlice(0)::last`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
        ],
    ),
)
```
