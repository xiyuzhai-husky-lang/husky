EntitySynTreePresheet {
    module_path: `std::logic`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [],
    },
    use_one_rules: UseOneRules(
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
                state: UseOneRuleState::Unresolved,
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        7,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `logic`,
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                6,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 1,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                4,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 2,
                        },
                    ),
                },
            ),
        ],
    },
    errors: [],
}