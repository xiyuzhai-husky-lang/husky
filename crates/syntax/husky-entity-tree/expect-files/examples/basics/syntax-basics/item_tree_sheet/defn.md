EntityTreeSheet {
    module_path: `syntax_basics::defn`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::Submodule(
                    SubmoduleSynNode {
                        syn_node_path: SubmoduleSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 6,
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn`,
                        ),
                        ast_idx: 1,
                        ident_token: IdentToken {
                            ident: `major_item`,
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
                                value: 6,
                            },
                        ),
                    ),
                ),
                ident: `major_item`,
                visibility: Scope::PubUnder(
                    `syntax_basics::defn`,
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `major_item`,
                visibility: Scope::PubUnder(
                    `syntax_basics::defn`,
                ),
                symbol: EntitySymbol::Submodule {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `syntax_basics::defn::major_item`,
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
}