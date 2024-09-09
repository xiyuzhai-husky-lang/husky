## `ConnectedComponentDistribution`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::i32`),
        ],
    ),
)
```

## `EffHoles`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
        ],
    ),
)
```

## `hole_tmpl`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `ConnectedComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist::BinaryImage28`),
        ],
    ),
)
```

## `horizontal_extend`

```rust
Some(
    Ok(
        [
            ItemPath(`core::raw_bits::r32`),
        ],
    ),
)
```

## `find_connected_components`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist::BinaryImage28`),
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
            ItemPath(`<#derive _ as core::clone::Clone(0)>::clone`),
            ItemPath(`core::raw_bits::r32(0)::ctz`),
            ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
            ItemPath(`mnist_classifier::connected_component::horizontal_extend`),
            ItemPath(`core::vec::Vec(0)::push`),
        ],
    ),
)
```

## `impl Visualize for ConnectedComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visualize`),
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
        ],
    ),
)
```

## `(ConnectedComponent as Visualize)::visualize`

```rust
Some(
    Ok(
        [
            ItemPath(`core::visual::Visual`),
            ItemPath(`<mnist::BinaryImage28 as core::visual::Visualize(0)>::visualize`),
        ],
    ),
)
```

## `impl ConnectedComponent`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
        ],
    ),
)
```

## `ConnectedComponent::raw_contours`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::raw_contour::find_raw_contours`),
        ],
    ),
)
```

## `ConnectedComponent::eff_holes`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::connected_component::EffHoles`),
            ItemPath(`core::vec::Vec(0)::collect_leashes`),
            ItemPath(`mnist_classifier::raw_contour::RawContour`),
            ItemPath(`mnist_classifier::connected_component::hole_tmpl`),
            ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
            ItemPath(`core::vec::Vec(0)::push`),
        ],
    ),
)
```

## `ConnectedComponent::max_hole_ilen`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`core::vec::Vec(0)::ilen`),
        ],
    ),
)
```

## `ConnectedComponent::max_row_span`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`core::num::i32`),
            ItemPath(`core::raw_bits::r32(0)::span`),
            ItemPath(`core::num::i32(0)::max`),
        ],
    ),
)
```

## `ConnectedComponent::row_span_sum`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`core::raw_bits::r32(0)::span`),
        ],
    ),
)
```

## `ConnectedComponent::distribution`

```rust
Some(
    Ok(
        [
            ItemPath(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
            ItemPath(`core::raw_bits::r32(0)::co`),
        ],
    ),
)
```

## `ConnectedComponent::upper_mass`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `ConnectedComponent::lower_mass`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
        ],
    ),
)
```

## `ConnectedComponent::top_k_row_span_sum`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::i32`),
            ItemPath(`core::num::f32`),
            ItemPath(`core::raw_bits::r32(0)::span`),
        ],
    ),
)
```

## `ConnectedComponent::top_k_row_right_mass_sum`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::i32`),
            ItemPath(`core::num::f32`),
            ItemPath(`core::raw_bits::r32(0)::right_mass`),
        ],
    ),
)
```
