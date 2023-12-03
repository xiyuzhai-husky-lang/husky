Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        3,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 3,
                        rule_idx: UseOneRuleIdx(
                            0,
                        ),
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
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        2,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 2,
                        rule_idx: UseOneRuleIdx(
                            1,
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
                },
            ),
            None,
            None,
        ],
    },
)