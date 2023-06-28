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
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 0,
                    use_expr_idx: 2,
                    visibility: Scope::Pub,
                    variant: UseExprRuleVariant::Parent {
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
                    state: UseExprRuleState::Erroneous,
                },
            ],
        ),
        use_all_rules: UseAllRules(
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