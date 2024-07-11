```rust
EntityTreeSheet {
    module_path: ModulePath(`core::default`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TraitPath(`core::default::Default`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `Default`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                        block: DefnBlock::Trait {
                            path: TraitPath(`core::default::Default`),
                            items: Some(
                                TraitItems {
                                    ast_idx_range: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::default::Default`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `Default`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `Default`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Trait(
                        TraitPath(`core::default::Default`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `array`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::array),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::array`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `backend`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::backend),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::backend`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `basic`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::basic),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::basic`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `clone`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::clone),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::clone`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `cmp`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::cmp),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::cmp`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `default`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::default),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::default`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `frontend`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::frontend),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::frontend`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fmt`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::fmt),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::fmt`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `logic`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::logic),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::logic`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `marker`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::marker),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::marker`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `mem`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::mem),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::mem`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `num`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::num),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::num`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ops`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::ops),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `option`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::option),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::option`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `prelude`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::prelude),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::prelude`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `raw_bits`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::raw_bits),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::raw_bits`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `result`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::result),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::result`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `task`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::task),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::task`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `vec`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::vec),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::vec`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `slice`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::slice),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::slice`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `str`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::str),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::str`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `visual`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`core::visual),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`core::visual`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `bool`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::bool`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::bool`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::bool`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `never`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::never`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::never`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::never`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::never`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `unit`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::unit`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::unit`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::unit`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Trait`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Trait`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Trait`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Trait`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Trait`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Module`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Module`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Module`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Module`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Module`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Lifetime`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Lifetime`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Lifetime`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Lifetime`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Lifetime`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Place`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Place`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Place`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Place`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Place`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Universe`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::basic::Universe`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::basic::Universe`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 0,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::basic::Universe`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Universe`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Default`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::default::Default`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::default::Default`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 1,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::default::Default`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::default::Default`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i8`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i8`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i8`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i16`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i16`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i16`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i32`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i64`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i64`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i128`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::i128`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i128`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `isize`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u8`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u8`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u8`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u16`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u16`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u16`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u32`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u64`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u64`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u128`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::u128`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u128`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `usize`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::usize`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `f32`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `f64`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f64`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `r32`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
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
                                        use_expr_idx: 9,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::raw_bits::r32`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Debug`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
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
                                        use_expr_idx: 12,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::fmt::Debug`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `IsTask`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::task::IsTask`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::task::IsTask`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 5,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::task::IsTask`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::task::IsTask`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Task`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`core::task::Task`, `TypeVar`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`core::task::Task`, `TypeVar`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 5,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`core::task::Task`, `TypeVar`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`core::task::Task`, `TypeVar`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `task`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`core::task::task`, `StaticVar`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`core::task::task`, `StaticVar`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 5,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`core::task::task`, `StaticVar`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`core::task::task`, `StaticVar`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Visualize`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::visual::Visualize`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::visual::Visualize`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 6,
                                        use_expr_idx: 18,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::visual::Visualize`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::visual::Visualize`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Visual`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::visual::Visual`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::visual::Visual`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 6,
                                        use_expr_idx: 18,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::visual::Visual`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::visual::Visual`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Copy`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::marker::Copy`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::marker::Copy`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 7,
                                        use_expr_idx: 21,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::marker::Copy`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Sized`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::marker::Sized`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::marker::Sized`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 7,
                                        use_expr_idx: 21,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::marker::Sized`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Sized`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Clone`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::clone::Clone`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::clone::Clone`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 8,
                                        use_expr_idx: 24,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::clone::Clone`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Option`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::option::Option`, `Enum`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::option::Option`, `Enum`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 9,
                                        use_expr_idx: 27,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::option::Option`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Some`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
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
                                                        ItemPathId(
                                                            Id {
                                                                value: 87,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 87,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 3,
                                                use_expr_idx: 2,
                                            },
                                        ),
                                        path: PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 87,
                                                    },
                                                ),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 9,
                                        use_expr_idx: 27,
                                    },
                                ),
                                path: PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `None`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
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
                                                        ItemPathId(
                                                            Id {
                                                                value: 88,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 88,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 3,
                                                use_expr_idx: 2,
                                            },
                                        ),
                                        path: PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 88,
                                                    },
                                                ),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 9,
                                        use_expr_idx: 27,
                                    },
                                ),
                                path: PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 88,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `PartialEq`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialEq`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialEq`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 10,
                                        use_expr_idx: 30,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::PartialEq`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialEq`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Eq`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Eq`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Eq`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 10,
                                        use_expr_idx: 30,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::Eq`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Eq`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `PartialOrd`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialOrd`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::PartialOrd`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 10,
                                        use_expr_idx: 30,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::PartialOrd`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialOrd`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Ord`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Ord`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::cmp::Ord`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 10,
                                        use_expr_idx: 30,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Trait(
                                        TraitPath(`core::cmp::Ord`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Slice`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 11,
                                        use_expr_idx: 33,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::slice::Slice`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::slice::Slice`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `CyclicSlice`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 11,
                                        use_expr_idx: 33,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vec`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::vec::Vec`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 12,
                                        use_expr_idx: 36,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::vec::Vec`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`core`),
                                ),
                                ast_idx: 22,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::vec::Vec`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`core::default`),
                        ),
                        ast_idx: 1,
                        use_expr_idx: 0,
                    },
                ),
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 1,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    ModulePath(`core::default`),
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
                        0..1,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: ModulePath(`core`),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: ModulePath(`core`),
                is_same_crate: true,
                ast_idx: 1,
                use_expr_idx: 0,
                visibility: Scope::PubUnder(
                    ModulePath(`core::default`),
                ),
                progress: Ok(
                    65,
                ),
            },
        ],
    ),
    errors: [],
}
```