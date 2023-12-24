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
                    DeclarativeTerm(`(independent v0: Type) -> fn((v0) -> core::option::Option v0`),
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
                    DeclarativeTerm(`(independent v0: Type) -> core::option::Option v0`),
                ),
            ),
        ],
    ),
]