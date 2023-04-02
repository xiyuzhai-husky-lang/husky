Ok(
    EntityTreeSheet {
        module_path: `std::logic`,
        symbols: EntitySymbolTable(
            [],
        ),
        impl_blocks: [],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 0,
                    use_expr_idx: 2,
                    visibility: Visibility::Pub,
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
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
                OriginalEntityTreeError::UnresolvedIdent(
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