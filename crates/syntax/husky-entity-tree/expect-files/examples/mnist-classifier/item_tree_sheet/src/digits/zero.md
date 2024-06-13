```rust
EntityTreeSheet {
    module_path: ModulePath(`mnist_classifier::digits::zero`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::digits::zero::open_one_match`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 25,
                        ident_token: IdentToken {
                            ident: `open_one_match`,
                            token_idx: TokenIdx(
                                6,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::digits::zero::open_one_match`, `Val`, (0)),
                    ),
                ),
                ident: `open_one_match`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 26,
                        ident_token: IdentToken {
                            ident: `almost_closed`,
                            token_idx: TokenIdx(
                                19,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        1..3,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `almost_closed`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::digits::zero::is_zero`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits`),
                        ),
                        ast_idx: 27,
                        ident_token: IdentToken {
                            ident: `is_zero`,
                            token_idx: TokenIdx(
                                50,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        11..24,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::digits::zero::is_zero`, `Val`, (0)),
                    ),
                ),
                ident: `is_zero`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits`),
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `open_one_match`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `almost_closed`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `is_zero`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits`),
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `zero`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::zero),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `one`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::one),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::one`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `six`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::six),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::six`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `three`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::three),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::three`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `four`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::four),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::four`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `five`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::five),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::five`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `seven`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::seven),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::seven`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `eight`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::eight),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::eight`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `nine`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::nine),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::nine`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `two`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits::two),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits::two`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_one`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 10,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `connected_component`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::connected_component),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist_classifier::connected_component`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::connected_component`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `raw_contour`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::raw_contour),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist_classifier::raw_contour`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `geom2d`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::geom2d),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist_classifier::geom2d`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::geom2d`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `line_segment_sketch`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::line_segment_sketch`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::fermi),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist_classifier::fermi`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::fermi`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `digits`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::digits`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::major),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist_classifier::major`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::major`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `main`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::main`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::main`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::main`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Class`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`malamute::Class`, `Enum`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::Class`, `Enum`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 23,
                                        use_expr_idx: 18,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::Class`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAll`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 23,
                                        use_expr_idx: 18,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAllResult`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 23,
                                        use_expr_idx: 18,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `narrow_down`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`malamute::narrow_down`, `Ritchie(
                                                    Gn,
                                                )`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`malamute::narrow_down`, `Ritchie(
                                                    Gn,
                                                )`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 23,
                                        use_expr_idx: 18,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`malamute::narrow_down`, `Ritchie(
                                            Gn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `task`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Submodule {
                                            submodule_item_path: SubmoduleItemPath(`mnist::task),
                                        },
                                        path: PrincipalEntityPath::Module(
                                            ModulePath(`mnist::task`),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 24,
                                        use_expr_idx: 20,
                                    },
                                ),
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`mnist::task`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist::task`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `MnistLabel`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 24,
                                        use_expr_idx: 20,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::MnistLabel`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryImage28`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist::BinaryImage28`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist::BinaryImage28`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 24,
                                        use_expr_idx: 20,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::BinaryImage28`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryGrid28`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 24,
                                        use_expr_idx: 20,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `input`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist::input`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist::input`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 24,
                                        use_expr_idx: 20,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist::input`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist::input`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_six`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 11,
                                use_expr_idx: 2,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_zero`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 12,
                                use_expr_idx: 5,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_two`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 13,
                                use_expr_idx: 8,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_three`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 14,
                                use_expr_idx: 11,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_five`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 15,
                                use_expr_idx: 14,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_seven`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 16,
                                use_expr_idx: 17,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_eight`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 17,
                                use_expr_idx: 20,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_nine`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 18,
                                use_expr_idx: 23,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `connected_components`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 17,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::connected_components`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_connected_component`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 17,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ignored_connected_components_row_span_sum_sum`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 17,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contours`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 17,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contour`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 17,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_line_segment_sketch`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 17,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_concave_components`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 17,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_one`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 10,
                                                use_expr_idx: 0,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `FermiMatchResult`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 19,
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi_match`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 19,
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 20,
                                        use_expr_idx: 9,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_raw_contours`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 20,
                                        use_expr_idx: 9,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentStroke`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 21,
                                        use_expr_idx: 12,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentSketch`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 21,
                                        use_expr_idx: 12,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 163,
                                                use_expr_idx: 0,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 21,
                                        use_expr_idx: 12,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                                            Fn,
                                                        )`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                                            Fn,
                                                        )`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 163,
                                                use_expr_idx: 0,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 21,
                                        use_expr_idx: 12,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 165,
                                                use_expr_idx: 4,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 21,
                                        use_expr_idx: 12,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponentDistribution`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 22,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `EffHoles`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 22,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponent`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 22,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_connected_components`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 22,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_six`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 11,
                                                use_expr_idx: 2,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_zero`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 12,
                                                use_expr_idx: 5,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_two`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 13,
                                                use_expr_idx: 8,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_three`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 14,
                                                use_expr_idx: 11,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_five`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 15,
                                                use_expr_idx: 14,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_seven`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 16,
                                                use_expr_idx: 17,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_eight`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 17,
                                                use_expr_idx: 20,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_nine`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::Use(
                                            UseSymbol {
                                                original_symbol: EntitySymbol::MajorItem {
                                                    major_item_path: MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                    ),
                                                },
                                                path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                    ),
                                                ),
                                                visibility: Scope::Pub,
                                                ast_idx: 18,
                                                use_expr_idx: 23,
                                            },
                                        ),
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::PubUnder(
                                            ModulePath(`mnist_classifier`),
                                        ),
                                        ast_idx: 18,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::digits`),
                                ),
                                ast_idx: 19,
                                use_expr_idx: 26,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::digits::zero`),
                        ),
                        ast_idx: 24,
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
                ast_idx: 24,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Super(
                        SuperToken {
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
                        EntitySymbol::RootSuperModule {
                            current_module_path: ModulePath(`mnist_classifier::digits::zero`),
                            super_module_path: ModulePath(`mnist_classifier::digits`),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: ModulePath(`mnist_classifier::digits`),
                is_same_crate: true,
                ast_idx: 24,
                use_expr_idx: 0,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::digits::zero`),
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