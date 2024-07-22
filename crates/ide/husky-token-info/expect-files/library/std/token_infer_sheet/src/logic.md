```rust
Ok(
    TokenInfoSheet {
        token_infos_list: [
            [],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::UseExpr(
                        2,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 2,
                        rule_idx: OnceUseRuleIdx(
                            0,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::UniversalPrelude {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                            ),
                        },
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::UseExpr(
                        1,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 1,
                        rule_idx: OnceUseRuleIdx(
                            1,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::PackageDependencyOrSelfLib {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`core::logic`),
                                    ),
                                },
                            ),
                        },
                    },
                },
            ],
            [],
            [],
        ],
    },
)
```