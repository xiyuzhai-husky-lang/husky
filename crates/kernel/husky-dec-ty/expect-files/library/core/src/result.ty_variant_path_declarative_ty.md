```rust
[
    (
        TypePath(`core::result::Result`, `Enum`),
        [
            (
                TypeVariantPath(`core::result::Result::Ok`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((s) -> core::result::Result s t`),
                ),
            ),
            (
                TypeVariantPath(`core::result::Result::Err`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((t) -> core::result::Result s t`),
                ),
            ),
        ],
    ),
]
```