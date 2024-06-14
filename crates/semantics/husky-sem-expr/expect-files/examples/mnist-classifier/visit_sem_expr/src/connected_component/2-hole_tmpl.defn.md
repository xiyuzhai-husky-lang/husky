```rust
Some(
    [
        "let len = ct.contour_len",
        "require len > 4.0",
        "len + 0.0",
        "let len = ct.contour_len\n    require len > 4.0\n    // ad hoc\n    len + 0.0",
    ],
)
```