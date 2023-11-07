[
    SemaExprRegion {
        [salsa id]: 16,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::logic::Prop`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::logic::Prop`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 9,
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 17,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::logic::Prop`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::logic::Prop`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `Prop`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `Prop`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `P`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    10,
                                ),
                                access_end: None,
                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Q`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: True,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                2..3,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::Traits,
                        syn_expr_idx: 1,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::Traits,
                        syn_expr_idx: 2,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 10,
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
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
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
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
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 6,
                        },
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 18,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: RegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::logic::Prop`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::logic::Prop`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `Prop`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `Prop`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegion {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `P`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Const,
                                access_start: RegionalTokenIdx(
                                    10,
                                ),
                                access_end: None,
                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                    syn_attrs: TemplateParameterSynAttrs {
                                        syn_attrs: [],
                                    },
                                    annotated_variance_token: None,
                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Q`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                        (
                            TemplateTypeParameter,
                            ArenaIdxRange(
                                2..3,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::Traits,
                        syn_expr_idx: 1,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::Traits,
                        syn_expr_idx: 2,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 11,
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
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
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
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
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                        Some(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ],
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
                        data: [],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 8,
                        },
                    ),
                ),
            ),
        },
    },
]