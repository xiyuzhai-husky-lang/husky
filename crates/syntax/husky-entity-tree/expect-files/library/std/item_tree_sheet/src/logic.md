```rust
EntityTreeSheet {
    module_path: ModulePath(`std::logic`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [],
    },
    item_symbol_table: EntitySymbolTable(
        [],
    ),
    impl_block_syn_node_table: [],
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
                                3,
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
                        EntitySymbol::UniversalPrelude {
                            item_path: PrincipalEntityPath::Module(
                                ModulePath(`core`),
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 0,
                use_expr_idx: 1,
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
                        0..1,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            ModulePath(`core`),
                        ),
                        EntitySymbol::UniversalPrelude {
                            item_path: PrincipalEntityPath::Module(
                                ModulePath(`core`),
                            ),
                        },
                    ),
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
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: ModulePath(`core::logic`),
                is_same_crate: false,
                ast_idx: 0,
                use_expr_idx: 0,
                visibility: Scope::Pub,
                progress: Ok(
                    0,
                ),
            },
        ],
    ),
    errors: [],
}
```