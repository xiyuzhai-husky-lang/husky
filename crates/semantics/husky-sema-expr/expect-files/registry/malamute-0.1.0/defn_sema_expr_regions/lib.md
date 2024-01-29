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
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::default::Default`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `default`,
                                                                item_kind: AssociatedFunctionFn,
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
                                                                            module_path: `malamute`,
                                                                            trai_path: TraitPath(`core::default::Default`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                    ident: `default`,
                                                                    item_kind: AssociatedFunctionFn,
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `No`,
                                                        index: U8(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                        1,
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    2,
                    (
                        SemaExprIdx(
                            2,
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
                inherited_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Type`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`t`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`a`),
                        ),
                    },
                ],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        1,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`t`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        1,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`a`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`OneVsAll t a`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 2,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`OneVsAll t a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`t`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expectation(
                                            1,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expectation(
                                            1,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
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
                                                EtherealTerm(`OneVsAll t a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                EtherealTerm(`OneVsAll t a`),
            ),
            self_ty: Some(
                EtherealTerm(`OneVsAll t a`),
            ),
        },
    },
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
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::Class`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `unveil`,
                                                                item_kind: AssociatedFunctionFn,
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
                                                                            module_path: `malamute`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`malamute::Class`, `Enum`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                    ident: `unveil`,
                                                                    item_kind: AssociatedFunctionFn,
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
                                SemaExprData::InheritedSynSymbol {
                                    ident: `one_vs_all`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 3,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `one_vs_all`,
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
                                        EtherealTerm(`OneVsAll t a`),
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
                                            EtherealTerm(`OneVsAll t a`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ident: `Break`,
                                                        index: U8(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                                        ident: `Known`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    4,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                        InheritedTemplateParameterSynSymbol::Constant {
                                            ident: `label`,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`t`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    4,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    5,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 14,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ident: `Continue`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    7,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    8,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            8,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    8,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
                                            ),
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
                                            3..4,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                        6,
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        9,
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_target: SemaExprIdx(
                                        1,
                                    ),
                                    eol_with_token: EolWithRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    case_branches: [
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    24,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 2,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    28,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    10,
                    (
                        SemaExprIdx(
                            10,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
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
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 96,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
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
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 96,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Type`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`t`),
                            ),
                        },
                    ),
                    SymbolType(
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
                                EtherealTerm(`OneVsAll t a`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`a`),
                        ),
                    },
                ],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Hollow(
                                                    HollowTerm(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`Class t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`ControlFlow Class t unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  Class t) -> ControlFlow Class t unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        3,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: Some(
                                                Const,
                                            ),
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`t`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: Some(
                                                    Const,
                                                ),
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`t`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`malamute::Class`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::Class`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`Class t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  t) -> Class t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Class t`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`Class t`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`Class t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        7,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`ControlFlow Class t unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            8,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  unit) -> ControlFlow Class t unit`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 9,
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
                                            EtherealTerm(`OneVsAll t a`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                2,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
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
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                5,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
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
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    4,
                                                ),
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
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`t`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coersion(
                                                ExpectCoersionOutcome {
                                                    coersion: Trivial(
                                                        TrivialFluffyCoersion {
                                                            expectee_place: Const,
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
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                5,
                                            ),
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
                                                EtherealTerm(`ControlFlow Class t unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Class t`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            6,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expectation(
                                            6,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                9,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                8,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
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
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
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
                                                EtherealTerm(`ControlFlow Class t unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`Class t`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
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
                                                EtherealTerm(`ControlFlow Class t unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                EtherealTerm(`ControlFlow Class t unit`),
            ),
            self_ty: Some(
                EtherealTerm(`Class t`),
            ),
        },
    },
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
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `unveil`,
                                                                item_kind: AssociatedFunctionFn,
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
                                                                            module_path: `malamute`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                    ident: `unveil`,
                                                                    item_kind: AssociatedFunctionFn,
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
                                SemaExprData::InheritedSynSymbol {
                                    ident: `one_vs_all_result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 3,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `one_vs_all_result`,
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
                                        EtherealTerm(`OneVsAllResult t a`),
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
                                            EtherealTerm(`OneVsAllResult t a`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ident: `Break`,
                                                        index: U8(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `Yes`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    4,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    5,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    3,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 14,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ident: `Break`,
                                                        index: U8(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    7,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    8,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 16,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `No`,
                                                        index: U8(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    11,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    12,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            13,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                13,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    6,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 22,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ident: `Continue`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    14,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                    Explicit(
                                                        FluffyTerm {
                                                            place: None,
                                                            base: Hollow(
                                                                HollowTerm(
                                                                    15,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            17,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    16,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                17,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionRitchieCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    9,
                                                ),
                                                coersion_outcome: Some(
                                                    ExpectCoersionOutcome {
                                                        coersion: Trivial(
                                                            TrivialFluffyCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            16,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                16,
                                            ),
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
                                            4..5,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    21,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                        4,
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        7,
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Eval {
                                    sema_expr_idx: SemaExprIdx(
                                        10,
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            16,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                SemaStmtData::Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_target: SemaExprIdx(
                                        1,
                                    ),
                                    eol_with_token: EolWithRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    case_branches: [
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    21,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 2,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        },
                                        SemaCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    38,
                                                ),
                                            ),
                                            case_pattern_sema_obelisk: CasePatternSemaSyndicate {
                                                syn_pattern_root: CaseSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 3,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..1,
                                                ),
                                            },
                                            heavy_arrow_token: HeavyArrowRegionalToken(
                                                RegionalTokenIdx(
                                                    42,
                                                ),
                                            ),
                                            stmts: SemaStmtIdxRange(
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    11,
                    (
                        SemaExprIdx(
                            11,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternExprTypeInfo {
                    ty: Ok(
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
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 107,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
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
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 107,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternExprTypeInfo {
                    ty: Ok(
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
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 107,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`Type`),
                            ),
                        },
                    ),
                    SymbolType(
                        FluffyTerm {
                            place: Some(
                                Const,
                            ),
                            base: FluffyTermBase::Ethereal(
                                EtherealTerm(`t`),
                            ),
                        },
                    ),
                    SymbolType(
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
                                EtherealTerm(`OneVsAllResult t a`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`a`),
                        ),
                    },
                ],
                current_syn_symbol_map: [],
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: [],
                    },
                    hollow_terms: HollowTerms {
                        entries: [
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Hollow(
                                                    HollowTerm(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`OneVsAll t a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        2,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`ControlFlow OneVsAll t a unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  OneVsAll t a) -> ControlFlow OneVsAll t a unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        3,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`t`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        3,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`a`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`OneVsAll t a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        5,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    13,
                                                ),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Hollow(
                                                    HollowTerm(
                                                        13,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`OneVsAll t a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        5,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`ControlFlow OneVsAll t a unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            7,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  OneVsAll t a) -> ControlFlow OneVsAll t a unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`t`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`t`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: Any,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`a`),
                                            ),
                                        },
                                    ),
                                    constraints: [],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    11,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    12,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`OneVsAll t a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        8,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`OneVsAll t a`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`OneVsAll t a`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`OneVsAll t a`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        8,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FluffyTerm {
                                                place: None,
                                                base: FluffyTermBase::Ethereal(
                                                    EtherealTerm(`unit`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::TypeOntology {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    refined_path: Right(
                                        CustomTypePath(
                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    14,
                                                ),
                                            ),
                                        },
                                        FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    15,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`ControlFlow OneVsAll t a unit`),
                                ),
                            },
                            HollowTermEntry {
                                data: HollowTermData::Ritchie {
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    params: [
                                        FluffyRitchieParameter::Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Move,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Hollow(
                                                        HollowTerm(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                16,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                    EtherealTerm(`fn((move  unit) -> ControlFlow OneVsAll t a unit`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 16,
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
                                            EtherealTerm(`OneVsAllResult t a`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                2,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
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
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
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
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                6,
                                            ),
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
                                                EtherealTerm(`ControlFlow OneVsAll t a unit`),
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
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`OneVsAll t a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            4,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            4,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`t`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            5,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expectation(
                                            5,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                10,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                9,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
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
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    7,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                13,
                                            ),
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
                                                EtherealTerm(`ControlFlow OneVsAll t a unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                9,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`OneVsAll t a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            11,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`t`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            12,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                11,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expectation(
                                            12,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                12,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                17,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Hollow(
                                                            HollowTerm(
                                                                16,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FluffyRitchieParameter::Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Move,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Hollow(
                                                                            HollowTerm(
                                                                                15,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
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
                                            base: FluffyTermBase::Hollow(
                                                HollowTerm(
                                                    15,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
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
                                                EtherealTerm(`ControlFlow OneVsAll t a unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                16,
                                            ),
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
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`OneVsAll t a`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expectation(
                                            18,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                14,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtype {
                                        expected: FluffyTerm {
                                            place: None,
                                            base: FluffyTermBase::Ethereal(
                                                EtherealTerm(`unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expectation(
                                            18,
                                        ),
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                15,
                                            ),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
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
                                                EtherealTerm(`ControlFlow OneVsAll t a unit`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                2,
                                            ),
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
                EtherealTerm(`ControlFlow OneVsAll t a unit`),
            ),
            self_ty: Some(
                EtherealTerm(`OneVsAll t a`),
            ),
        },
    },
]