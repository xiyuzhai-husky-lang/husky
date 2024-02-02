[
    (
        TypePath(`core::ops::ControlFlow`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                ident: `Continue`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((t) -> core::ops::ControlFlow s t`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                ident: `Break`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (t: Type) -> fn((s) -> core::ops::ControlFlow s t`),
                ),
            ),
        ],
    ),
]