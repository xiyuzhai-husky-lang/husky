```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`semantics_basics`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`semantics_basics::some_neural_network`, `Ritchie(
                                        Gn,
                                    )`, (0)),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`semantics_basics`),
                                ),
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `some_neural_network`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: FormPath(`semantics_basics::some_neural_network`, `Ritchie(
                                        Gn,
                                    )`),
                                    body: Some(
                                        FormBody {
                                            ast_idx_range: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`semantics_basics::some_neural_network`, `Ritchie(
                                    Gn,
                                )`, (0)),
                            ),
                        ),
                        ident: `some_neural_network`,
                        visibility: Scope::PubUnder(
                            ModulePath(`semantics_basics`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `some_neural_network`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`semantics_basics`),
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`semantics_basics::some_neural_network`, `Ritchie(
                                    Gn,
                                )`),
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