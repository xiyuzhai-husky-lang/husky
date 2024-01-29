[
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `visualize`,
                                                                item_kind: MethodFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
            path: SynNodeRegionPath::Defn(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TraitForTypeItem(
                        TraitForTypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TraitForTypeItem(
                                        TraitForTypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitForTypeItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::AssociatedItem(
                                                            AssociatedItemPathData::TraitForTypeItem(
                                                                TraitForTypeItemPathData {
                                                                    impl_block: TraitForTypeImplBlock {
                                                                        data: TraitForTypeImplBlockPathData {
                                                                            module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                    ident: `visualize`,
                                                                    item_kind: MethodFn,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
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
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::SelfValue(
                                    RegionalTokenIdx(
                                        1,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`ConvexComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`ConvexComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`ConvexComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `line_segments`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                        signature: FluffyFieldSignature::PropsStruct {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Leash CyclicSlice LineSegmentStroke`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash CyclicSlice LineSegmentStroke`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice LineSegmentStroke`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::MethodFnCall {
                                    self_argument_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `visualize`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                path: TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 191,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            Application(
                                                                EtherealTermApplication(
                                                                    Id {
                                                                        value: 54,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 88,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EtherealTermSymbol(
                                                                Id {
                                                                    value: 3,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Application(
                                                                            EtherealTermApplication(
                                                                                Id {
                                                                                    value: 54,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
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
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Visual`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Visual`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Visual`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Visual`),
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
                    data: [
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coersion(
                                            ExpectCoersionOutcome {
                                                coersion: Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Visual`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    4,
                    (
                        SemaExprIdx(
                            4,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
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
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`ConvexComponent`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash CyclicSlice LineSegmentStroke`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Visual`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Visual`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Visual`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Visual`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                },
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
                EtherealTerm(`Visual`),
            ),
            self_ty: Some(
                EtherealTerm(`ConvexComponent`),
            ),
        },
    },
]