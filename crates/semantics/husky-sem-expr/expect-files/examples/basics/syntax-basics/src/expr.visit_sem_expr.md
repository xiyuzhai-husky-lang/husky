## `nested` decl

```rust
Some(
    [],
)
```

## `nested` defn

```rust
Some(
    [
        "1",
        "1",
        "{\n        1\n    }",
        "let t = {\n        1\n    }",
        "let t = {\n        1\n    }",
    ],
)
```

## `closure_inline` decl

```rust
Some(
    [],
)
```

## `closure_inline` defn

```rust
Some(
    [
        "i32",
        "x",
        "1",
        "x + 1",
        "|x: i32| x + 1",
        "let t = |x: i32| x + 1",
        "let t = |x: i32| x + 1",
    ],
)
```

## `closure_nested` decl

```rust
Some(
    [],
)
```

## `closure_nested` defn

```rust
Some(
    [
        "i32",
        "x",
        "1",
        "x + 1",
        "x + 1",
        "{\n        x + 1\n    }",
        "|x: i32| {\n        x + 1\n    }",
        "let t = |x: i32| {\n        x + 1\n    }",
        "let t = |x: i32| {\n        x + 1\n    }",
    ],
)
```
