[
    (
        TypePath(`malamute::Class`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                ident: `Known`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v0: Type) -> fn((v0) -> malamute::Class v0`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                ident: `Unknown`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v0: Type) -> malamute::Class v0`),
                ),
            ),
        ],
    ),
    (
        TypePath(`malamute::OneVsAll`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `Yes`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v1: Type) -> (independent v0: v1) -> malamute::OneVsAll v1 v0`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `No`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v1: Type) -> (independent v0: v1) -> malamute::OneVsAll v1 v0`),
                ),
            ),
        ],
    ),
    (
        TypePath(`malamute::OneVsAllResult`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ident: `ConfidentYes`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v1: Type) -> (independent v0: v1) -> malamute::OneVsAllResult v1 v0`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ident: `ConfidentNo`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v1: Type) -> (independent v0: v1) -> malamute::OneVsAllResult v1 v0`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ident: `Unconfident`,
                                index: U8(
                                    2,
                                ),
                            },
                        ),
                    },
                ),
                Ok(
                    DeclarativeTerm(`(independent v1: Type) -> (independent v0: v1) -> malamute::OneVsAllResult v1 v0`),
                ),
            ),
        ],
    ),
]