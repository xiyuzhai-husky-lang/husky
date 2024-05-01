```rust
[
    SemExprRegion {
        path: RegionPath::Decl(
            ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`core::task::Task`, `Extern`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::Decl(
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::task::Task`, `Extern`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [],
                },
            ),
            sem_expr_roots: [],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EthTerm(`Task`),
            ),
        },
    },
]
```