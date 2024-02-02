[
    (
        TypePath(`core::result::Result`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`core::result::Result`, `Enum`),
                                ident: `Ok`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v1: Type) -> (independent v0: Type) -> fn((v1) -> core::result::Result v1 v0`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`core::result::Result`, `Enum`),
                                ident: `Err`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v1: Type) -> (independent v0: Type) -> fn((v0) -> core::result::Result v1 v0`),
                ),
            ),
        ],
    ),
]