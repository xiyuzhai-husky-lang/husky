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
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((s) -> core::result::Result s t`),
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
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((t) -> core::result::Result s t`),
                ),
            ),
        ],
    ),
]