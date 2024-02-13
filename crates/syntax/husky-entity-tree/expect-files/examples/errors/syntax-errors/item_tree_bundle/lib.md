EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: `syntax_errors`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_errors`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `ast`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        ),
                        ident: `ast`,
                        visibility: Scope::PubUnder(
                            `syntax_errors`,
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_errors`,
                                ),
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `uses`,
                                    token_idx: TokenIdx(
                                        4,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        ),
                        ident: `uses`,
                        visibility: Scope::PubUnder(
                            `syntax_errors`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ast`,
                        visibility: Scope::PubUnder(
                            `syntax_errors`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_errors::ast`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `uses`,
                        visibility: Scope::PubUnder(
                            `syntax_errors`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_errors::uses`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_errors::ast`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_errors::ast`,
                                ),
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `A`,
                                    token_idx: TokenIdx(
                                        4,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`syntax_errors::ast::A`, `Struct`),
                                    variants: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ident: `A`,
                        visibility: Scope::PubUnder(
                            `syntax_errors::ast`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `A`,
                        visibility: Scope::PubUnder(
                            `syntax_errors::ast`,
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`syntax_errors::ast::A`, `Struct`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [
                (
                    ImplBlockSynNodePath::TypeImplBlock(
                        TypeImplBlockSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 129,
                                },
                            ),
                        ),
                    ),
                    ImplBlockSynNode::TypeImplBlock(
                        TypeImplBlockSynNode {
                            syn_node_path: TypeImplBlockSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 129,
                                    },
                                ),
                            ),
                            ast_idx: 4,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            ty_expr: 1,
                            items: TypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    1..2,
                                ),
                            },
                        },
                    ),
                ),
            ],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_errors::uses`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [],
            },
            item_symbol_table: EntitySymbolTable(
                [],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
    ],
    principal_item_path_expr_arena: Arena {
        data: [
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `A`,
                        token_idx: TokenIdx(
                            8,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`syntax_errors::ast::A`, `Struct`),
                    ),
                ),
            },
        ],
    },
}