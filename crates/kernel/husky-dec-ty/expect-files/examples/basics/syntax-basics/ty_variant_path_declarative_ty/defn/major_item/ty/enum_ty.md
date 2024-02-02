[
    (
        TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `UnitVariant`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `PropsVariantWithOneField`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`fn((core::num::i32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `PropsVariantWithTwoFields`,
                                index: U8(
                                    2,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`fn((core::num::i32, core::num::f32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `TupleVariantWithOneField`,
                                index: U8(
                                    3,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`fn((core::num::i32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `TupleVariantWithTwoFields`,
                                index: U8(
                                    4,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`fn((core::num::i32, core::num::f32) -> syntax_basics::defn::major_item::ty::enum_ty::A`),
                ),
            ),
        ],
    ),
]