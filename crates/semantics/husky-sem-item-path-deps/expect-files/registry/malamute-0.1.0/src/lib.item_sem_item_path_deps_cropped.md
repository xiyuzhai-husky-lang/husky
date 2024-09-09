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
        [],
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
        [],
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
        [],
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
            ItemPath(`malamute::OneVsAllResult`),
        ],
    ),
)
```

## ``narrow_down`::#dep`

```rust
Some(
    Ok(
        [],
    ),
)
```

## `impl Default for OneVsAll`

```rust
Some(
    Ok(
        [
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
            ItemPath(`malamute::Class::Known`),
        ],
    ),
)
```

## `impl Unveil for OneVsAll`

```rust
Some(
    Ok(
        [
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
            ItemPath(`malamute::OneVsAll::Yes`),
            ItemPath(`malamute::OneVsAll::No`),
        ],
    ),
)
```
