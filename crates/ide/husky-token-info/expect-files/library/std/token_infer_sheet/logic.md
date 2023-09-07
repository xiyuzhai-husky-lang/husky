Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 3,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::UniversalPrelude {
                            item_path: PrincipalEntityPath::Module(
                                `core`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: OnceUseRuleIdx(
                    1,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::PackageDependency {
                            item_path: PrincipalEntityPath::Module(
                                `core::logic`,
                            ),
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)