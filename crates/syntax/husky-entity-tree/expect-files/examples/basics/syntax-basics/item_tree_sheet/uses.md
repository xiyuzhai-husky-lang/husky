EntityTreeSheet {
    module_path: `syntax_basics::uses`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `ast`,
                visibility: Scope::PubUnder(
                    `syntax_basics::uses`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::ast`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `syntax_basics::ast`,
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `uses`,
                visibility: Scope::PubUnder(
                    `syntax_basics::uses`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::uses`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `syntax_basics::uses`,
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `defn`,
                visibility: Scope::PubUnder(
                    `syntax_basics::uses`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::defn`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `syntax_basics::defn`,
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `expr`,
                visibility: Scope::PubUnder(
                    `syntax_basics::uses`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::expr`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `syntax_basics::expr`,
                        ),
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        ast_idx: 1,
                        use_expr_idx: 1,
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
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `syntax_basics::uses`,
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
                            root_module_path: `syntax_basics`,
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: `syntax_basics`,
                is_same_crate: true,
                ast_idx: 1,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    `syntax_basics::uses`,
                ),
                progress: Ok(
                    4,
                ),
            },
        ],
    ),
    errors: [],
}