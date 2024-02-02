[
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::marker::Copy`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Decl(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::marker::Copy`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FlyTermRegion {
                terms: FlyTerms {
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
            return_ty: None,
            self_ty: Some(
                EthTerm(`t`),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::marker::Sized`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Decl(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::marker::Sized`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FlyTermRegion {
                terms: FlyTerms {
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
            return_ty: None,
            self_ty: Some(
                EthTerm(`t`),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockSynNodePathData {
                                        path: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `core::marker`,
                                                trai_path: TraitPath(`core::marker::Copy`),
                                                ty_sketch: TypeSketch::DeriveAny,
                                                disambiguator: 0,
                                            },
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: SynNodeRegionPath::Decl(
                ItemSynNodePath::ImplBlock(
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `core::marker`,
                                                    trai_path: TraitPath(`core::marker::Copy`),
                                                    ty_sketch: TypeSketch::DeriveAny,
                                                    disambiguator: 0,
                                                },
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::marker::Copy`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Ethereal(
                                        EthTerm(`Trait`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Ethereal(
                                            EthTerm(`Trait`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [],
                },
            ),
            sema_expr_roots: [
                (
                    1,
                    (
                        SemaExprIdx(
                            1,
                        ),
                        Trait,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        1,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    Trait(
                                        TraitPath(
                                            ItemPathId(
                                                Id {
                                                    value: 36,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FlyTermRegion {
                terms: FlyTerms {
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
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Ethereal(
                                            EthTerm(`Trait`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EthTerm(`t`),
            ),
        },
    },
]