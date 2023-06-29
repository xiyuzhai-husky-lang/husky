Ok(
    EntityTreeSheet {
        module_path: `std::logic`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [],
        },
        entity_symbol_table: EntitySymbolTable(
            [],
        ),
        impl_block_node_table: [],
        once_use_rules: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 0,
                    use_expr_idx: 2,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `core`,
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
                    state: OnceUseRuleState::Erroneous,
                },
            ],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [],
        ),
        errors: [
            EntityTreeError::Original(
                OriginalEntityTreeError::UnresolvedRootIdent(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                ),
            ),
        ],
    },
)