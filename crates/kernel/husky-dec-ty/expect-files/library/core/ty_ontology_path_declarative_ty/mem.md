```rust
[
    (
        TypePath(`core::mem::Ref`, `Extern`),
        Ok(
            DecTerm(`covariant core::basic::Lifetime -> covariant Type -> Type`),
        ),
    ),
    (
        TypePath(`core::mem::RefMut`, `Extern`),
        Ok(
            DecTerm(`covariant core::basic::Lifetime -> invariant Type -> Type`),
        ),
    ),
    (
        TypePath(`core::mem::Leash`, `Extern`),
        Ok(
            DecTerm(`covariant Type -> Type`),
        ),
    ),
    (
        TypePath(`core::mem::At`, `Extern`),
        Ok(
            DecTerm(`independent core::basic::Place -> independent Type -> Type`),
        ),
    ),
]
```