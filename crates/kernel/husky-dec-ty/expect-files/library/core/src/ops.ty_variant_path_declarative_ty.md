```rust
[
    (
        TypePath(`core::ops::ControlFlow`, `Enum`),
        [
            (
                TypeVariantPath(`core::ops::ControlFlow::Continue`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((t) -> core::ops::ControlFlow s t`),
                ),
            ),
            (
                TypeVariantPath(`core::ops::ControlFlow::Break`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((s) -> core::ops::ControlFlow s t`),
                ),
            ),
        ],
    ),
]
```