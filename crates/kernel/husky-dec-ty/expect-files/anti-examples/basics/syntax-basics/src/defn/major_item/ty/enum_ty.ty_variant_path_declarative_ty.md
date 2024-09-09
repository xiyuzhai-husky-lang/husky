```rust
[
    (
        TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
        [
            (
                TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
                Ok(
                    DecTerm(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                Ok(
                    DecTerm(`fn((core::num::i32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                Ok(
                    DecTerm(`fn((core::num::i32, core::num::f32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                Ok(
                    DecTerm(`fn((core::num::i32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                Ok(
                    DecTerm(`fn((core::num::i32, core::num::f32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
        ],
    ),
]
```