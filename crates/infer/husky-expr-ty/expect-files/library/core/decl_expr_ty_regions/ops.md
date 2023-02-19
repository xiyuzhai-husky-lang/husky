[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TraitPath(`core::ops::Add`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [],
        },
        expr_local_terms: ArenaMap {
            data: [],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Type`),
            ),
        ],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
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