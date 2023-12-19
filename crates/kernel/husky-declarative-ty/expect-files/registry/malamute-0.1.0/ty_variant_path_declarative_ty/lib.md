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