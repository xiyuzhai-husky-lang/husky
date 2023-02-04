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
                opt_expectation: None,
                resolved_ty: Ok(
                    Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                ),
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
                opt_expectation: None,
                resolved_ty: Err(
                    Derived(
                        ResolvedTypeUninitialized,
                    ),
                ),
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Err(
                    Derived(
                        ResolvedTypeUninitialized,
                    ),
                ),
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Err(
                    Derived(
                        ResolvedTypeUninitialized,
                    ),
                ),
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