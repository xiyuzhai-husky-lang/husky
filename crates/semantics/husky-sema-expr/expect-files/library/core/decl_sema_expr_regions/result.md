[
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`core::result::Result`, `Enum`),
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
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Type(
                                        TypeSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::result::Result`, `Enum`),
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
                current_syn_symbol_map: [
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
                                EtherealTerm(`Type`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                ],
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
            return_ty: None,
            self_ty: Some(
                EtherealTerm(`Result t s`),
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
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
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
                                                    module_path: `core::result`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
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
                                    path_expr_idx: 3,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::ops::Unveil`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`invariant Type -> Trait`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`invariant Type -> Trait`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> independent Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> independent Type -> Trait`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Trait`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `T2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `T2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
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
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> Trait`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> Trait`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `E2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `E2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Trait`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Trait`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> independent Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `T1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `T1`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    3,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::CurrentSynSymbol {
                                    ident: `E1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `E1`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        11,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
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
                    7,
                    (
                        SemaExprIdx(
                            7,
                        ),
                        Trait,
                    ),
                ),
                (
                    12,
                    (
                        SemaExprIdx(
                            12,
                        ),
                        SelfType,
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
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    Trait(
                                        TraitPath(
                                            ItemPathId(
                                                Id {
                                                    value: 71,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        2,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        3,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 32,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        4,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 9,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 35,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        6,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        7,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 38,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        8,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        9,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 8,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        10,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        11,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        12,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 31,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [
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
                                EtherealTerm(`Type`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_syn_symbol_map: [
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                ],
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
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyOriginal,
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
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`invariant Type -> Trait`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Trait`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Invariant,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Trait`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyOriginal,
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
                                            EtherealTerm(`independent Type -> independent Type -> Trait`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`independent Type -> Trait`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`independent Type -> Trait`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::AnyOriginal,
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
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> Trait`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Trait`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Trait`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
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
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Trait`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`independent Type -> Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`independent Type -> Type`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                TermUniverse(
                                                    1,
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
            return_ty: None,
            self_ty: Some(
                EtherealTerm(`Result t s`),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
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
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `Continue`,
                                                                item_kind: AssociatedType,
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
            path: SynNodeRegionPath::Decl(
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
                                                                            module_path: `core::result`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                    ident: `Continue`,
                                                                    item_kind: AssociatedType,
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
                                    ident: `E2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 4,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                        InheritedTemplateParameterSynSymbol::Type {
                                            ident: `E2`,
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
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
                        AssociatedTypeTerm,
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
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
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
                                EtherealTerm(`Type`),
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
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
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
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                TermUniverse(
                                                    1,
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
            return_ty: None,
            self_ty: Some(
                EtherealTerm(`Result t s`),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
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
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`core::result::Result`, `Enum`),
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
            path: SynNodeRegionPath::Decl(
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
                                                                            module_path: `core::result`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`core::result::Result`, `Enum`),
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
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> independent Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `T2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                        InheritedTemplateParameterSynSymbol::Type {
                                            ident: `T2`,
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `E2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    inherited_syn_symbol_idx: 4,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                        InheritedTemplateParameterSynSymbol::Type {
                                            ident: `E2`,
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
                                        EtherealTerm(`Type`),
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
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> independent Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `T1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                        InheritedTemplateParameterSynSymbol::Type {
                                            ident: `T1`,
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        6,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`independent Type -> Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::InheritedSynSymbol {
                                    ident: `E1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    inherited_syn_symbol_idx: 3,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                        InheritedTemplateParameterSynSymbol::Type {
                                            ident: `E1`,
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                SemaExprData::FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
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
                    5,
                    (
                        SemaExprIdx(
                            5,
                        ),
                        ExplicitParameterType,
                    ),
                ),
                (
                    10,
                    (
                        SemaExprIdx(
                            10,
                        ),
                        ReturnType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    None,
                ],
            },
            sema_expr_terms: [
                (
                    SemaExprIdx(
                        1,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        2,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 9,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        3,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 34,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        4,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        6,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        7,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 8,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        8,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 29,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        9,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Symbol(
                                    EtherealTermSymbol(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                (
                    SemaExprIdx(
                        10,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 31,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
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
                                EtherealTerm(`Type`),
                            ),
                        },
                    ),
                ],
                current_syn_symbol_map: [
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
                                EtherealTerm(`Result t s`),
                            ),
                        },
                    ),
                ],
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
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
                        ),
                    },
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`t`),
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
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
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
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`independent Type -> Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`independent Type -> Type`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
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
                                            EtherealTerm(`independent Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
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
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                TermUniverse(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
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
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> independent Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`independent Type -> Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`independent Type -> Type`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`independent Type -> Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                        return_ty: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`Type`),
                                                            ),
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::CurryDestination(
                                    ExpectCurryDestination {
                                        curry_destination: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: Expectation::EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Type`),
                                        ),
                                    },
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsSort(
                                                TermUniverse(
                                                    1,
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
            return_ty: None,
            self_ty: Some(
                EtherealTerm(`Result t s`),
            ),
        },
    },
]