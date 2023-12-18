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
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v0: Type) -> (independent v0: Type) -> fn((v0) -> core::ops::ControlFlow v0 v0`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                ident: `Break`,
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v0: Type) -> (independent v0: Type) -> fn((v0) -> core::ops::ControlFlow v0 v0`),
                ),
            ),
        ],
    ),
]