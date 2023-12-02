EntitySynTreeSheet {
    module_path: `std::logic`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [],
    },
    item_symbol_table: EntitySymbolTable(
        [],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: UseOneRules(
        [
            UseOneRule {
                ast_idx: 1,
                use_expr_idx: 3,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        2..3,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::UniversalPrelude {
                            item_path: PrincipalEntityPath::Module(
                                `core`,
                            ),
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 1,
                use_expr_idx: 2,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `logic`,
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        1..2,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::PackageDependency {
                            item_path: PrincipalEntityPath::Module(
                                `core::logic`,
                            ),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: `core::logic`,
                is_same_crate: false,
                ast_idx: 1,
                use_expr_idx: 1,
                visibility: Scope::Pub,
                progress: Ok(
                    0,
                ),
            },
        ],
    ),
    errors: [],
}