EntitySynTreePresheet {
    module_path: `syntax_errors::uses`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [],
    },
    use_one_rules: UseOneRules(
        [],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        6,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::SelfMod(
                        SelfModToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                3,
                            ),
                        ),
                    ),
                    children: Err(
                        UseExprError::Original(
                            OriginalUseExprError::CannotUseCrateForChild {
                                crate_token: CrateToken {
                                    token_idx: TokenIdx(
                                        4,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ],
    },
}