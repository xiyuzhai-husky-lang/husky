[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
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
        expr_ty_infos: [],
        expr_local_terms: ArenaMap {
            data: [],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
]