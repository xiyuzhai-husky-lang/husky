```rust
[
    (
        TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 11,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`fn((core::num::i32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 12,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`fn((core::num::i32, core::num::f32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 13,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`fn((core::num::i32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`fn((core::num::i32, core::num::f32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
        ],
    ),
]
```