[
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Err(
                ExprTermError::Derived(
                    DerivedExprTermError::TypeInfoNotInferred,
                ),
            ),
            Err(
                ExprTermError::Derived(
                    DerivedExprTermError::TypeInfoNotInferred,
                ),
            ),
            Err(
                ExprTermError::Derived(
                    DerivedExprTermError::TypeInfoNotInferred,
                ),
            ),
            Err(
                ExprTermError::Derived(
                    DerivedExprTermError::TypeInfoNotInferred,
                ),
            ),
            Err(
                ExprTermError::Derived(
                    DerivedExprTermError::TypeInfoNotInferred,
                ),
            ),
            Err(
                ExprTermError::Derived(
                    DerivedExprTermError::TypeInfoNotInferred,
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`LineSegmentSketch`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`i32`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
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
        return_ty: Some(
            EtherealTerm(`bool`),
        ),
        self_ty: None,
    },
]