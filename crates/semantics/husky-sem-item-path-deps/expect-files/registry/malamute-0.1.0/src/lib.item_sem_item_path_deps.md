## `Class`

```rust
Some(
    Ok(
        [],
    ),
)
```

## ``Class`::#derive`

```rust
Some(
    Ok(
        [
            ItemPath(`core::fmt::Debug`),
            ItemPath(`core::clone::Clone`),
            ItemPath(`core::marker::Copy`),
        ],
    ),
)
```

## `Class::Known`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `Class::Unknown`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `OneVsAll`

```rust
Some(
    Ok(
        [],
    ),
)
```

## ``OneVsAll`::#derive`

```rust
Some(
    Ok(
        [
            ItemPath(`core::fmt::Debug`),
            ItemPath(`core::clone::Clone`),
            ItemPath(`core::marker::Copy`),
        ],
    ),
)
```

## `OneVsAll::Yes`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `OneVsAll::No`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `OneVsAllResult`

```rust
Some(
    Ok(
        [],
    ),
)
```

## ``OneVsAllResult`::#derive`

```rust
Some(
    Ok(
        [
            ItemPath(`core::fmt::Debug`),
            ItemPath(`core::clone::Clone`),
            ItemPath(`core::marker::Copy`),
        ],
    ),
)
```

## `OneVsAllResult::ConfidentYes`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `OneVsAllResult::ConfidentNo`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `OneVsAllResult::Unconfident`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `narrow_down`

```rust
Some(
    Ok(
        [
            ItemPath(`core::num::f32`),
            ItemPath(`core::num::i32`),
            ItemPath(`malamute::OneVsAllResult`),
        ],
    ),
)
```

## ``narrow_down`::#dep`

```rust
Some(
    Ok(
        [
            ItemPath(`core::task::Task`),
        ],
    ),
)
```

## `impl Default for OneVsAll`

```rust
Some(
    Ok(
        [
            ItemPath(`core::default::Default`),
            ItemPath(`malamute::OneVsAll`),
        ],
    ),
)
```

## `(OneVsAll as Default)::default`

```rust
Some(
    Ok(
        [
            ItemPath(`malamute::OneVsAll::No`),
        ],
    ),
)
```

## `impl Unveil for Class`

```rust
Some(
    Ok(
        [
            ItemPath(`core::ops::Unveil`),
            ItemPath(`malamute::OneVsAll`),
            ItemPath(`malamute::Class`),
        ],
    ),
)
```

## `(Class as Unveil)::Output`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `(Class as Unveil)::unveil`

```rust
Some(
    Ok(
        [
            ItemPath(`malamute::OneVsAll`),
            ItemPath(`core::ops::ControlFlow`),
            ItemPath(`core::ops::ControlFlow::Break`),
            ItemPath(`malamute::Class::Known`),
            ItemPath(`core::ops::ControlFlow::Continue`),
        ],
    ),
)
```

## `impl Unveil for OneVsAll`

```rust
Some(
    Ok(
        [
            ItemPath(`core::ops::Unveil`),
            ItemPath(`malamute::OneVsAllResult`),
            ItemPath(`malamute::OneVsAll`),
        ],
    ),
)
```

## `(OneVsAll as Unveil)::Output`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `(OneVsAll as Unveil)::unveil`

```rust
Some(
    Ok(
        [
            ItemPath(`malamute::OneVsAllResult`),
            ItemPath(`core::ops::ControlFlow`),
            ItemPath(`core::ops::ControlFlow::Break`),
            ItemPath(`malamute::OneVsAll::Yes`),
            ItemPath(`malamute::OneVsAll::No`),
            ItemPath(`core::ops::ControlFlow::Continue`),
        ],
    ),
)
```
