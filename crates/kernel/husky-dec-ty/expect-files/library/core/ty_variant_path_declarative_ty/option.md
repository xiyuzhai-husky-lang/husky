[
    (
        TypePath(`core::option::Option`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                ident: `Some`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`(independent (t: Type) -> fn((t) -> core::option::Option t`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                ident: `None`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`(independent (t: Type) -> core::option::Option t`),
                ),
            ),
        ],
    ),
]