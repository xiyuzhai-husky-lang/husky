Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeSynDefn::PropsStruct(
                    PropsStructTypeSynDefn {
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        decl: PropsStructTypeSynDecl {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_parameters: [],
                            fields: [
                                PropsFieldSyndicate {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    ty_syn_expr_idx: 5,
                                    initialization: None,
                                    variable: 1,
                                },
                                PropsFieldSyndicate {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            14,
                                        ),
                                    ),
                                    ty_syn_expr_idx: 9,
                                    initialization: None,
                                    variable: 2,
                                },
                            ],
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 1,
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                opd: 2,
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                opd: 6,
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 7,
                                                argument_expr_idx: 8,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `matches`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `others`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                13,
                                                            ),
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                FieldVariable {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 248,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    ty_expr_idx: 5,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                FieldVariable {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 434,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                    ty_expr_idx: 9,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `others`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 9,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::FunctionFn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                        decl: FunctionFnFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                            template_parameters: [],
                            parenate_parameters: [
                                ParenateSynParameterData::Ordinary {
                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                ParenateSynParameterData::Ordinary {
                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                        syn_pattern_expr_idx: 2,
                                    },
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                    ty: 11,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 12,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                opd: 1,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                                opd: 6,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    23,
                                                ),
                                                opd: 8,
                                            },
                                            SynExprData::Ritchie {
                                                ritchie_kind: Type(
                                                    Fn,
                                                ),
                                                ritchie_kind_regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                lpar_token: LparRegionalToken(
                                                    RegionalTokenIdx(
                                                        18,
                                                    ),
                                                ),
                                                parameter_ty_items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 7,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                                light_arrow_token: Some(
                                                    LightArrowRegionalToken(
                                                        RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    ),
                                                ),
                                                return_ty_syn_expr_idx: Some(
                                                    9,
                                                ),
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 5,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 10,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `FermiMatchResult`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `concave_components`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `templates`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                None,
                                                None,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `concave_components`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `templates`,
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                None,
                                                None,
                                            ],
                                        },
                                    },
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `concave_components`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `templates`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 4,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    ty_expr_idx: 11,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 2,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 11,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 12,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                        (
                                            2,
                                            2,
                                        ),
                                    ],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                25,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::MajorItem(
                                                            MajorItemSynNodePath::Fugitive(
                                                                FugitiveSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExprData::List {
                                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                                    8,
                                                                ),
                                                                items: [],
                                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Tilde,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                                opd: 1,
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::ExplicitApplication {
                                                                function_expr_idx: 2,
                                                                argument_expr_idx: 3,
                                                            },
                                                            SynExprData::List {
                                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                                    14,
                                                                ),
                                                                items: [],
                                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Tilde,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    19,
                                                                ),
                                                                opd: 6,
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 3,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Option,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    23,
                                                                ),
                                                                opd: 8,
                                                            },
                                                            SynExprData::Ritchie {
                                                                ritchie_kind: Type(
                                                                    Fn,
                                                                ),
                                                                ritchie_kind_regional_token_idx: RegionalTokenIdx(
                                                                    17,
                                                                ),
                                                                lpar_token: LparRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        18,
                                                                    ),
                                                                ),
                                                                parameter_ty_items: [
                                                                    SynCommaListItem {
                                                                        syn_expr_idx: 7,
                                                                        comma_regional_token_idx: None,
                                                                    },
                                                                ],
                                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                                    21,
                                                                ),
                                                                light_arrow_token: Some(
                                                                    LightArrowRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            22,
                                                                        ),
                                                                    ),
                                                                ),
                                                                return_ty_syn_expr_idx: Some(
                                                                    9,
                                                                ),
                                                            },
                                                            SynExprData::FunctionApplicationOrCall {
                                                                function: 5,
                                                                template_arguments: None,
                                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                                    16,
                                                                ),
                                                                items: [
                                                                    SynCommaListItem {
                                                                        syn_expr_idx: 10,
                                                                        comma_regional_token_idx: None,
                                                                    },
                                                                ],
                                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                                    25,
                                                                ),
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 4,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            20,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `f32`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            24,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `FermiMatchResult`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            28,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            data: [
                                                                SynPatternExpr::Ident {
                                                                    symbol_modifier_tokens: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `concave_components`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                                SynPatternExpr::Ident {
                                                                    symbol_modifier_tokens: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `templates`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [
                                                                None,
                                                                None,
                                                            ],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [
                                                                SynPatternSymbol::Atom(
                                                                    1,
                                                                ),
                                                                SynPatternSymbol::Atom(
                                                                    2,
                                                                ),
                                                            ],
                                                        },
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `concave_components`,
                                                                    1,
                                                                ),
                                                            ],
                                                            [
                                                                (
                                                                    `templates`,
                                                                    2,
                                                                ),
                                                            ],
                                                        ],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [
                                                                None,
                                                                None,
                                                            ],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_syn_symbol_arena: Arena {
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                    access_end: None,
                                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                                        ident: `concave_components`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        13,
                                                                    ),
                                                                    access_end: None,
                                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                                        ident: `templates`,
                                                                        pattern_symbol_idx: 2,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            (
                                                                OrdinaryParenateParameter {
                                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                                        syn_pattern_expr_idx: 1,
                                                                    },
                                                                    ty_expr_idx: 4,
                                                                },
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                            (
                                                                OrdinaryParenateParameter {
                                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                                        syn_pattern_expr_idx: 2,
                                                                    },
                                                                    ty_expr_idx: 11,
                                                                },
                                                                ArenaIdxRange(
                                                                    2..3,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    syn_pattern_expr_roots: [
                                                        SynPatternExprRoot {
                                                            kind: SynPatternExprRootKind::Parenate,
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                        SynPatternExprRoot {
                                                            kind: SynPatternExprRootKind::Parenate,
                                                            syn_pattern_expr_idx: 2,
                                                        },
                                                    ],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ExplicitParameterType,
                                                            syn_expr_idx: 4,
                                                        },
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ExplicitParameterType,
                                                            syn_expr_idx: 11,
                                                        },
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 12,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [
                                                        (
                                                            1,
                                                            1,
                                                        ),
                                                        (
                                                            2,
                                                            2,
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::InheritedSynSymbol {
                                                    ident: `concave_components`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    inherited_syn_symbol_idx: 1,
                                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `concave_components`,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `collect_leashes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Prefix {
                                                    opr: Tilde,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    opd: 3,
                                                },
                                                SynExprData::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                                SynExprData::Prefix {
                                                    opr: Option,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    opd: 4,
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 5,
                                                    argument_expr_idx: 6,
                                                },
                                                SynExprData::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                },
                                                SynExprData::InheritedSynSymbol {
                                                    ident: `templates`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    inherited_syn_symbol_idx: 2,
                                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                SynExprData::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        10,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 10,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    ropd: 11,
                                                },
                                                SynExprData::InheritedSynSymbol {
                                                    ident: `templates`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        34,
                                                    ),
                                                    inherited_syn_symbol_idx: 2,
                                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        36,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        10,
                                                    ),
                                                },
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 13,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 14,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        38,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `others`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `template`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                    current_syn_symbol_idx: 4,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 17,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `pop_with_largest_opt_f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            44,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        45,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 18,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        47,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 16,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `push`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            40,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        41,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 19,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        48,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        52,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `others`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::FunctionApplicationOrCall {
                                                    function: 21,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        51,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 22,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    53,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            syn_expr_idx: 23,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        55,
                                                    ),
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        3..7,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `ConcaveComponent`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `FermiMatchResult`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                50,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 3,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                33,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 15,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 20,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 2,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            13,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                19,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 8,
                                                },
                                                SynStmtData::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 10,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        11,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    for_loop_var_symbol_idx: 3,
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    30,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        1..3,
                                                    ),
                                                },
                                                SynStmtData::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    result: 24,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `others`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        11,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `matches`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `template`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                32,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    Move,
                                                    None,
                                                ],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    SynPatternSymbol::Atom(
                                                        1,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        2,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        3,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `others`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `matches`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `template`,
                                                        3,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                    Mut,
                                                    None,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        modifier: None,
                                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                                            ident: `concave_components`,
                                                        },
                                                    },
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            2,
                                                        ),
                                                        modifier: None,
                                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                                            ident: `templates`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    56,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    56,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `matches`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    49,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LoopVariable {
                                                            ident: `i`,
                                                            expr_idx: 10,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            33,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    49,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `template`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    LetPattern {
                                                        pattern: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 2,
                                                        },
                                                        ty: 7,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    LoopVariable,
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 2,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 3,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtType,
                                                syn_expr_idx: 7,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 15,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 20,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 24,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 25,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                            (
                                                2,
                                                2,
                                            ),
                                            (
                                                3,
                                                4,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynDefn::ImplBlock(
            ImplBlockSynDecl::Type(
                TypeImplBlockSynDecl {
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::fermi`,
                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        disambiguator: 0,
                    },
                    template_parameters: [],
                    self_ty_expr: SelfTypeSyndicate {
                        expr: 1,
                    },
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: SynNodeRegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TypeImplBlock(
                                        TypeImplBlockSynNodePath {
                                            path: TypeImplBlockPath {
                                                module_path: `mnist_classifier::fermi`,
                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                ident: `FermiMatchResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                            symbol_region: SynSymbolRegionData {
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
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::SelfType,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                },
            ),
        ),
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TypeItem(
                TypeItemSynDefn::MemoizedField(
                    TypeMemoizedFieldSynDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldSynDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `norm`,
                                item_kind: MemoizedField,
                            },
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    ident: `FermiMatchResult`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                symbol_region: SynSymbolRegionData {
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
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodeDataPath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `norm`,
                                                            item_kind: MemoizedField,
                                                        },
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                18,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: SynNodeRegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    ident: `FermiMatchResult`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                                MajorItemPath::Type(
                                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                symbol_region: SynSymbolRegionData {
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
                                                                syn_expr_roots: [
                                                                    SynExprRoot {
                                                                        kind: SynExprRootKind::SelfType,
                                                                        syn_expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                                syn_pattern_to_current_syn_symbol_map: [],
                                                            },
                                                        },
                                                    ),
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodeDataPath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::fermi`,
                                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `norm`,
                                                                            item_kind: MemoizedField,
                                                                        },
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
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                                        ident: `f32`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    syn_pattern_expr_roots: [],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodeDataPath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `norm`,
                                                                item_kind: MemoizedField,
                                                            },
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
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::SelfValue(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                SynExprData::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                SynExprData::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ropd: 6,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::SelfValue(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                ),
                                                SynExprData::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 10,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 11,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 13,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 14,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    ropd: 15,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..5,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `f32`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Eval {
                                                    expr_idx: 16,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmtData::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 5,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        6,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    for_loop_var_symbol_idx: 2,
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    18,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                SynStmtData::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    result: 17,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `norm`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                ],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    SynPatternSymbol::Atom(
                                                        1,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `norm`,
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    36,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `norm`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            19,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    34,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LoopVariable {
                                                            ident: `i`,
                                                            expr_idx: 5,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    LetPattern {
                                                        pattern: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    LoopVariable,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtType,
                                                syn_expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 16,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 17,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 18,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TypeItem(
                TypeItemSynDefn::MemoizedField(
                    TypeMemoizedFieldSynDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `rel_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldSynDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `rel_norm`,
                                item_kind: MemoizedField,
                            },
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    ident: `FermiMatchResult`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                symbol_region: SynSymbolRegionData {
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
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodeDataPath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `rel_norm`,
                                                            item_kind: MemoizedField,
                                                        },
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                18,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: SynNodeRegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    ident: `FermiMatchResult`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                                MajorItemPath::Type(
                                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                symbol_region: SynSymbolRegionData {
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
                                                                syn_expr_roots: [
                                                                    SynExprRoot {
                                                                        kind: SynExprRootKind::SelfType,
                                                                        syn_expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                                syn_pattern_to_current_syn_symbol_map: [],
                                                            },
                                                        },
                                                    ),
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodeDataPath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::fermi`,
                                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `rel_norm`,
                                                                            item_kind: MemoizedField,
                                                                        },
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
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                                        ident: `f32`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    syn_pattern_expr_roots: [],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodeDataPath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `rel_norm`,
                                                                item_kind: MemoizedField,
                                                            },
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
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 38,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::SelfValue(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                SynExprData::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                SynExprData::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ropd: 6,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::SelfValue(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                ),
                                                SynExprData::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 10,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 11,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `rel_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 13,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 14,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    ropd: 15,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..5,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `f32`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Eval {
                                                    expr_idx: 16,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmtData::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 5,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        6,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    for_loop_var_symbol_idx: 2,
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    18,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                SynStmtData::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    result: 17,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `norm`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                ],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    SynPatternSymbol::Atom(
                                                        1,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `norm`,
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    36,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `norm`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            19,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    34,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LoopVariable {
                                                            ident: `i`,
                                                            expr_idx: 5,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    LetPattern {
                                                        pattern: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    LoopVariable,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtType,
                                                syn_expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 16,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 17,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 18,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TypeItem(
                TypeItemSynDefn::MemoizedField(
                    TypeMemoizedFieldSynDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `angle_change_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldSynDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `angle_change_norm`,
                                item_kind: MemoizedField,
                            },
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    ident: `FermiMatchResult`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                symbol_region: SynSymbolRegionData {
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
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodeDataPath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `angle_change_norm`,
                                                            item_kind: MemoizedField,
                                                        },
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                19,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: SynNodeRegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    ident: `FermiMatchResult`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                                MajorItemPath::Type(
                                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                symbol_region: SynSymbolRegionData {
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
                                                                syn_expr_roots: [
                                                                    SynExprRoot {
                                                                        kind: SynExprRootKind::SelfType,
                                                                        syn_expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                                syn_pattern_to_current_syn_symbol_map: [],
                                                            },
                                                        },
                                                    ),
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodeDataPath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::fermi`,
                                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `angle_change_norm`,
                                                                            item_kind: MemoizedField,
                                                                        },
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
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                                        ident: `f32`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    syn_pattern_expr_roots: [],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodeDataPath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `angle_change_norm`,
                                                                item_kind: MemoizedField,
                                                            },
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
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 39,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::SelfValue(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                SynExprData::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                SynExprData::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ropd: 6,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::SelfValue(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                ),
                                                SynExprData::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 10,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 11,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `abs`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        36,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 14,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 15,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    ropd: 16,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..5,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `f32`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Eval {
                                                    expr_idx: 17,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmtData::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 5,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        6,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    for_loop_var_symbol_idx: 2,
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    18,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                SynStmtData::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                    result: 18,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `norm`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                ],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    SynPatternSymbol::Atom(
                                                        1,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `norm`,
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    40,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `norm`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            19,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    38,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LoopVariable {
                                                            ident: `i`,
                                                            expr_idx: 5,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    LetPattern {
                                                        pattern: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    LoopVariable,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtType,
                                                syn_expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 17,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 18,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 19,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ],
)