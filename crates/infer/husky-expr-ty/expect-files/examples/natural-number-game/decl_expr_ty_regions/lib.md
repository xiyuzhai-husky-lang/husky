[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::Nat`, `Inductive`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::OddNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::EvenNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `natural_number_game`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `natural_number_game`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                        },
                    },
                    ident: `add`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
]