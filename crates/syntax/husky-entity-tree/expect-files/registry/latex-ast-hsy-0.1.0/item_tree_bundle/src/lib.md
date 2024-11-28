```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`latex_ast_hsy`),
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
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `LxAstId`,
                                    token_idx: TokenIdx(
                                        18,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                                    variants: None,
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
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `LxAstId`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`latex_ast_hsy::AST`, `StaticVar`, (0)),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `AST`,
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: MajorFormPath(`latex_ast_hsy::AST`, `StaticVar`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`latex_ast_hsy::AST`, `StaticVar`, (0)),
                            ),
                        ),
                        ident: `AST`,
                        visibility: Scope::Pub,
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `LxAstId`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `AST`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`latex_ast_hsy::AST`, `StaticVar`),
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
    ],
    principal_item_path_expr_arena: Arena {
        data: [],
    },
}
```