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
            ItemPath(`core::slice::CyclicSlice`),
            ItemPath(`mnist_classifier::geom2d::Point2d`),
            ItemPath(`core::slice::CyclicSlice(0)::first`),
            ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
            ItemPath(`core::slice::CyclicSlice(0)::last`),
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
            ItemPath(`core::num::f32`),
            ItemPath(`core::num::f32(0)::sqrt`),
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
            ItemPath(`core::num::f32`),
            ItemPath(`core::num::f32(0)::sqrt`),
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
            ItemPath(`core::num::i32`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
            ItemPath(`core::vec::Vec(0)::ilen`),
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
            ItemPath(`core::num::i32`),
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::norm`),
            ItemPath(`core::num::i32(0)::min`),
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
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`mnist_classifier::line_segment_sketch::extend_end`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
            ItemPath(`core::vec::Vec(0)::last`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::cross`),
            ItemPath(`core::num::f32(0)::abs`),
            ItemPath(`mnist_classifier::geom2d::Vector2d(0)::dot`),
            ItemPath(`core::slice::CyclicSlice(0)::start`),
            ItemPath(`mnist_classifier::line_segment_sketch::extend_start`),
            ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
            ItemPath(`core::vec::Vec(0)::pop`),
            ItemPath(`core::slice::CyclicSlice(0)::end`),
            ItemPath(`core::vec::Vec(0)::push`),
            ItemPath(`core::vec::Vec(0)::first`),
        ],
    ),
)
```

## `impl Visualize for LineSegmentStroke`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visualize`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        ],
    ),
)
```

## `(LineSegmentStroke as Visualize)::visualize`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visual`),
        ],
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
            ItemPath(`core::num::i32`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
            ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
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
            ItemPath(`core::visual::Visualize`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
        ],
    ),
)
```

## `(LineSegmentSketch as Visualize)::visualize`

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
            ItemPath(`core::vec::Vec(0)::ilen`),
            ItemPath(`core::num::f32(0)::min`),
            ItemPath(`core::num::f32(0)::max`),
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
            ItemPath(`core::num::f32`),
            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
            ItemPath(`mnist_classifier::line_segment_sketch::find_line_segments`),
        ],
    ),
)
```
