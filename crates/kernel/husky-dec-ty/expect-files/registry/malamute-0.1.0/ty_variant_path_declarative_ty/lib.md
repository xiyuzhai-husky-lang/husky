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
                    DecTerm(`(independent (t: Type) -> fn((t) -> malamute::Class t`),
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
                    DecTerm(`(independent (t: Type) -> malamute::Class t`),
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
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAll s a`),
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
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAll s a`),
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
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
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
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
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
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
                ),
            ),
        ],
    ),
]