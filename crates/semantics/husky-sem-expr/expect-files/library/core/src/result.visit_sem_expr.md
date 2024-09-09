## `Result` decl

```rust
Some(
    [],
)
```

## `Result` defn

```rust
None
```

## `Result::Ok` decl

```rust
Some(
    [
        "T",
    ],
)
```

## `Result::Ok` defn

```rust
None
```

## `Result::Err` decl

```rust
Some(
    [
        "E",
    ],
)
```

## `Result::Err` defn

```rust
None
```

## `impl Unveil for Result` decl

```rust
Some(
    [
        "crate::ops::Unveil",
        "Result",
        "crate::ops::Unveil Result",
        "T2",
        "crate::ops::Unveil Result T2",
        "E2",
        "crate::ops::Unveil Result T2 E2",
        "Result",
        "T1",
        "Result T1",
        "E1",
        "Result T1 E1",
    ],
)
```

## `impl Unveil for Result` defn

```rust
None
```

## `(Result as Unveil)::Continue` decl

```rust
Some(
    [
        "E2",
    ],
)
```

## `(Result as Unveil)::Continue` defn

```rust
None
```

## `(Result as Unveil)::unveil` decl

```rust
Some(
    [
        "Result",
        "T2",
        "Result T2",
        "E2",
        "Result T2 E2",
        "Result",
        "T1",
        "Result T1",
        "E1",
        "Result T1 E1",
    ],
)
```

## `(Result as Unveil)::unveil` defn

```rust
None
```
