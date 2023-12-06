EntitySynTreeSheet {
    module_path: `core::option`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`core::option::Option`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 6,
                        ident_token: IdentToken {
                            ident: `Option`,
                            token_idx: TokenIdx(
                                25,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::option::Option`, `Enum`),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        1..3,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Type(
                                        TypeSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::option::Option`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `Option`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `Option`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    module_item_path: MajorItemPath::Type(
                        TypePath(`core::option::Option`, `Enum`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `Some`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::TypeVariant {
                            ty_variant_path: TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 74,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 115,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 74,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 115,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 4,
                        use_expr_idx: 3,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `None`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::TypeVariant {
                            ty_variant_path: TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 74,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 116,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 74,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 116,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 4,
                        use_expr_idx: 3,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `array`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::array`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::array`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `basic`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::basic`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::basic`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `clone`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::clone`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::clone`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `cmp`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::cmp`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::cmp`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `default`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::default`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::default`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fmt`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::fmt`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::fmt`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `logic`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::logic`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::logic`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `marker`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::marker`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::marker`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `mem`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::mem`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::mem`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `num`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::num`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::num`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ops`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::ops`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::ops`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `option`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::option`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::option`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `prelude`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::prelude`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::prelude`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `raw_bits`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::raw_bits`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::raw_bits`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `result`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::result`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::result`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `vec`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::vec`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::vec`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `slice`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::slice`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::slice`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `str`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::str`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::str`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `visual`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `core::visual`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `core::visual`,
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `bool`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::bool`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::bool`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::bool`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `never`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::never`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::never`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::never`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::never`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `unit`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::unit`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::unit`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::unit`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Trait`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Trait`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Trait`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Trait`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Trait`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Module`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Module`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Module`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Module`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Module`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Lifetime`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Lifetime`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Lifetime`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Lifetime`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Lifetime`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Place`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Place`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Place`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Place`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Place`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i8`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::i8`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::i8`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i8`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i16`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::i16`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::i16`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i16`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i16`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i32`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::i32`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::i32`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i64`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::i64`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::i64`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i64`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i128`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::i128`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::i128`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i128`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i128`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `isize`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::isize`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::isize`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u8`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::u8`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::u8`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u8`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u8`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u16`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::u16`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::u16`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u16`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u16`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u32`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::u32`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::u32`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u64`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::u64`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::u64`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u64`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u128`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::u128`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::u128`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u128`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u128`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `usize`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::usize`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `f32`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::f32`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::f32`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `f64`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::num::f64`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::f64`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 2,
                                        use_expr_idx: 4,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `r32`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 3,
                                        use_expr_idx: 7,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::raw_bits::r32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Debug`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::fmt::Debug`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::fmt::Debug`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 4,
                                        use_expr_idx: 10,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::fmt::Debug`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Visualize`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::visual::Visualize`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::visual::Visualize`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 5,
                                        use_expr_idx: 13,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::visual::Visualize`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::visual::Visualize`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Html`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::visual::Html`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::visual::Html`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 5,
                                        use_expr_idx: 13,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::visual::Html`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::visual::Html`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Copy`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::marker::Copy`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::marker::Copy`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 6,
                                        use_expr_idx: 16,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::marker::Copy`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Sized`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::marker::Sized`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::marker::Sized`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 6,
                                        use_expr_idx: 16,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::marker::Sized`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Sized`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Clone`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::clone::Clone`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::clone::Clone`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 7,
                                        use_expr_idx: 19,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::clone::Clone`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Option`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::option::Option`, `Enum`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::option::Option`, `Enum`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 8,
                                        use_expr_idx: 22,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::option::Option`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Some`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::TypeVariant {
                                                    ty_variant_path: TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 74,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    ident: Ident(
                                                                        Coword(
                                                                            Id {
                                                                                value: 115,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                },
                                                path: PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 74,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    ident: Ident(
                                                                        Coword(
                                                                            Id {
                                                                                value: 115,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 4,
                                                use_expr_idx: 3,
                                            },
                                        ),
                                        path: PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 74,
                                                                    },
                                                                ),
                                                            ),
                                                            ident: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 115,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 8,
                                        use_expr_idx: 22,
                                    },
                                ),
                                path: PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 74,
                                                            },
                                                        ),
                                                    ),
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 115,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 74,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 115,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `None`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::TypeVariant {
                                                    ty_variant_path: TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 74,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    ident: Ident(
                                                                        Coword(
                                                                            Id {
                                                                                value: 116,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                },
                                                path: PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 74,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    ident: Ident(
                                                                        Coword(
                                                                            Id {
                                                                                value: 116,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 4,
                                                use_expr_idx: 3,
                                            },
                                        ),
                                        path: PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 74,
                                                                    },
                                                                ),
                                                            ),
                                                            ident: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 116,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 8,
                                        use_expr_idx: 22,
                                    },
                                ),
                                path: PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 74,
                                                            },
                                                        ),
                                                    ),
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 116,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 74,
                                                    },
                                                ),
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 116,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `PartialEq`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialEq`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialEq`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 9,
                                        use_expr_idx: 25,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::PartialEq`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialEq`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Eq`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Eq`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Eq`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 9,
                                        use_expr_idx: 25,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::Eq`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Eq`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `PartialOrd`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialOrd`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialOrd`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 9,
                                        use_expr_idx: 25,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::PartialOrd`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialOrd`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Ord`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Ord`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Ord`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 9,
                                        use_expr_idx: 25,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::Ord`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Slice`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 10,
                                        use_expr_idx: 28,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::slice::Slice`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::slice::Slice`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `CyclicSlice`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 10,
                                        use_expr_idx: 28,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vec`,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 11,
                                        use_expr_idx: 31,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::vec::Vec`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `core`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::vec::Vec`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `core::option`,
                        ),
                        ast_idx: 3,
                        use_expr_idx: 1,
                    },
                ),
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: UseOneRules(
        [
            UseOneRule {
                ast_idx: 3,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        1..2,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 4,
                use_expr_idx: 4,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Option`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        3..4,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 4,
                use_expr_idx: 3,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::UseAllTypeVariants {
                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                },
                parent: Some(
                    MajorEntityPath::MajorItem(
                        MajorItemPath::Type(
                            TypePath(`core::option::Option`, `Enum`),
                        ),
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: None,
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: `core`,
                is_same_crate: true,
                ast_idx: 3,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    `core::option`,
                ),
                progress: Ok(
                    57,
                ),
            },
        ],
    ),
    errors: [],
}