[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: ReducedTerm(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
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
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::fermi`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: ReducedTerm(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
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
                        module_path: `mnist_classifier::fermi`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        },
                    },
                    ident: `norm`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: ReducedTerm(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
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
                        module_path: `mnist_classifier::fermi`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        },
                    },
                    ident: `rel_norm`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: ReducedTerm(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
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
                        module_path: `mnist_classifier::fermi`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        },
                    },
                    ident: `angle_change_norm`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: ReducedTerm(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
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
]