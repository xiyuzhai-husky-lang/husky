[
    SemaExprRegion {
        [salsa id]: 231,
        path: RegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `visualize`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Defn(
                AssociatedItem(
                    TraitForTypeItem(
                        TraitForTypeItemSynNodePath(
                            Id {
                                value: 26,
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
                                SelfType(
                                    RegionalTokenIdx(
                                        1,
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 60,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 399,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    field_dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        ty_path: TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                        signature: PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 75,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 75,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 160,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    method_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyTermDynamicDispatchIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    Id {
                                                                        value: 39,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_stmt_arena: SemaStmtArena(
                Arena {
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 39,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            syn_expr_root_sema_expr_idx_table: [
                (
                    4,
                    SemaExprIdx(
                        4,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
                    },
                    hollow_terms: HollowTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 75,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 39,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 39,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 39,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 39,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ImplicitlyConvertible(
                                                Trivial(
                                                    Todo,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 39,
                            },
                        ),
                    ),
                ),
            ),
            self_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
]