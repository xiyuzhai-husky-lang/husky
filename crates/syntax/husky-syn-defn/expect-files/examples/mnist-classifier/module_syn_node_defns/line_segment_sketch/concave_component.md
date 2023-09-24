Ok(
    [
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::PropsStruct(
                    PropsStructTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: PropsStructTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            lcurl: Ok(
                                PropsStructLcurlRegionalToken(
                                    LcurlRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                ),
                            ),
                            fields: Ok(
                                PunctuatedSmallList {
                                    elements: [
                                        PropsFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 220,
                                                        },
                                                    ),
                                                ),
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty_expr_idx: 2,
                                            initialization: None,
                                            variable: 1,
                                        },
                                        PropsFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 375,
                                                        },
                                                    ),
                                                ),
                                                regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                            ty_expr_idx: 6,
                                            initialization: None,
                                            variable: 2,
                                        },
                                    ],
                                    separators: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                9,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    ],
                                    phantom: PhantomData<husky_syn_decl::error::SynNodeDeclError>,
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    RcurlRegionalToken(
                                        RegionalTokenIdx(
                                            16,
                                        ),
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
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                opd: 1,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                opd: 3,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 4,
                                                argument_expr_idx: 5,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `LineSegmentSketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `CyclicSlice`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `LineSegmentStroke`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `line_segment_sketch`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `strokes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
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
                                                                    value: 220,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    ty_expr_idx: 2,
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
                                                                    value: 375,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    ty_expr_idx: 6,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 220,
                                                            },
                                                        ),
                                                    ),
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            },
                                            expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 375,
                                                            },
                                                        ),
                                                    ),
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                            },
                                            expr_idx: 6,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        SpecificParameterObelisk::Regular {
                                            syn_pattern_root: SynPatternRoot(
                                                1,
                                            ),
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 5,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                opd: 1,
                                            },
                                            SynExpr::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `line_segment_sketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `mnist_classifier::line_segment_sketch`,
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `LineSegmentSketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
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
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `line_segment_sketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                None,
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
                                                    `line_segment_sketch`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                None,
                                            ],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `line_segment_sketch`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    syn_pattern_root: SynPatternRoot(
                                                        1,
                                                    ),
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 5,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                65,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::MajorItem(
                                                            MajorItemSynNodePath::Fugitive(
                                                                FugitiveSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::Prefix {
                                                                opr: Tilde,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                                opd: 1,
                                                            },
                                                            SynExpr::List {
                                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                                items: [],
                                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                                    12,
                                                                ),
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 3,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 3,
                                                                argument_expr_idx: 4,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `line_segment_sketch`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::Module(
                                                                    `mnist_classifier::line_segment_sketch`,
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `LineSegmentSketch`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            13,
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
                                                            data: [
                                                                SynPatternExpr::Ident {
                                                                    symbol_modifier_tokens: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `line_segment_sketch`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [
                                                                None,
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
                                                                    `line_segment_sketch`,
                                                                    1,
                                                                ),
                                                            ],
                                                        ],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [
                                                                None,
                                                            ],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                        ident: `line_segment_sketch`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            (
                                                                ExplicitRegularParameter {
                                                                    syn_pattern_root: SynPatternRoot(
                                                                        1,
                                                                    ),
                                                                    ty_expr_idx: 2,
                                                                },
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ExplicitParameterType,
                                                            expr_idx: 2,
                                                        },
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 5,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 1,
                                                    argument_expr_idx: 2,
                                                },
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 5,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 6,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::CurrentSymbol {
                                                    ident: `L`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        34,
                                                    ),
                                                    opd: 10,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 13,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 14,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    41,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 15,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 11,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                    ropd: 12,
                                                },
                                                SynExpr::Prefix {
                                                    opr: Not,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                    opd: 16,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 17,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        36,
                                                    ),
                                                    ropd: 18,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        45,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 20,
                                                    opr: Attr,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        50,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ccv_start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `L`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        56,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        52,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 23,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        55,
                                                    ),
                                                    ropd: 24,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 25,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        53,
                                                    ),
                                                    ropd: 26,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        61,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `L`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        63,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        59,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 28,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        62,
                                                    ),
                                                    ropd: 29,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        68,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        70,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 32,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        67,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 33,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    69,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 34,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        71,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 30,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        60,
                                                    ),
                                                    ropd: 31,
                                                },
                                                SynExpr::Prefix {
                                                    opr: Not,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        65,
                                                    ),
                                                    opd: 35,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 36,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        64,
                                                    ),
                                                    ropd: 37,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        73,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 39,
                                                    opr: Incr,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        74,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        78,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        80,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::CurrentSymbol {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        76,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 41,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        79,
                                                    ),
                                                    ropd: 42,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 43,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        77,
                                                    ),
                                                    ropd: 44,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        82,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        88,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        90,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 49,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        91,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            92,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        96,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        98,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 50,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        93,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `cyclic_slice_leashed`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            94,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        95,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 51,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    97,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 52,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        99,
                                                    ),
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 47,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        87,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 48,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    89,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 53,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        100,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 46,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        83,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `push`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            84,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        85,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 54,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        101,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        102,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        104,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 56,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        103,
                                                    ),
                                                    ropd: 57,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        107,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        109,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::CurrentSymbol {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        105,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 59,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        108,
                                                    ),
                                                    ropd: 60,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 61,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        106,
                                                    ),
                                                    ropd: 62,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        111,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        8..16,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `ConcaveComponent`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_convex`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                38,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_convex`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                66,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `ConcaveComponent`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                86,
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
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 21,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 40,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 55,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::While {
                                                    while_token: WhileRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        38,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    72,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                75,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            45,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        81,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 58,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 63,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                1,
                                                            ),
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
                                                                3,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 4,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                2,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                13,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 7,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            21,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                3,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                24,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 8,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            26,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                4,
                                                            ),
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                29,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 9,
                                                },
                                                SynStmt::While {
                                                    while_token: WhileRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        19,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    44,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                5,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                5..6,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                49,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 22,
                                                },
                                                SynStmt::While {
                                                    while_token: WhileRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            51,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        27,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    57,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        4..8,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            110,
                                                        ),
                                                    },
                                                    result: 64,
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
                                                            ident: `concave_components`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `L`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        22,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `start`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                23,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        27,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `end`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                28,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `ccv_start`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                48,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    None,
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
                                                    SynPatternSymbol::Atom(
                                                        4,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        5,
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
                                                        `L`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `start`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `end`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `ccv_start`,
                                                        5,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                    None,
                                                    Mut,
                                                    Mut,
                                                    None,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        modifier: None,
                                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                                            ident: `line_segment_sketch`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    112,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `concave_components`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    112,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `L`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    112,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `start`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            29,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    112,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `end`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            49,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    112,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `ccv_start`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    LetPattern {
                                                        pattern: SynPatternRoot(
                                                            1,
                                                        ),
                                                        ty: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 3,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 4,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 7,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 9,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 21,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 22,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 40,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 55,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 58,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 63,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 64,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 65,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::ImplBlock(
            ImplBlockSynNodeDecl::TraitForType(
                TraitForTypeImplBlockSynNodeDecl {
                    syn_node_path: TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    impl_regional_token: ImplRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            1,
                        ),
                    },
                    template_parameter_decl_list: Ok(
                        None,
                    ),
                    trai_expr: TraitObelisk {
                        expr: 1,
                    },
                    for_token: ConnectionForRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            3,
                        ),
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeObelisk {
                            expr: 2,
                        },
                    ),
                    eol_colon: Ok(
                        EolRegionalToken::Colon(
                            EolColonRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                            },
                        ),
                    ),
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExpr::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::visual::Visualize`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::PrincipalEntityPath {
                                        path_expr_idx: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `Visualize`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::visual::Visualize`),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `ConcaveComponent`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
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
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: Trait,
                                    expr_idx: 1,
                                },
                                SynExprRoot {
                                    kind: SelfType,
                                    expr_idx: 2,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TraitForTypeItem(
                TraitForTypeItemSynNodeDefn::MethodFn(
                    TraitForTypeMethodFnSynNodeDefn {
                        syn_node_path: TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TraitForTypeMethodFnSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitForTypeItemPath {
                                        impl_block: TraitForTypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                            disambiguator: 0,
                                        },
                                        ident: `visualize`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Visualize`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::visual::Visualize`),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        4,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: Trait,
                                                        expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 2,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitForTypeItemPath {
                                                            impl_block: TraitForTypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::visual::Html`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Html`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::visual::Html`, `Extern`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                4,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePath {
                                                                                path: TraitForTypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                                    ty_sketch: TypeSketch::Path(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Trait(
                                                                                        TraitPath(`core::visual::Visualize`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 2,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `Visualize`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                                MajorItemPath::Trait(
                                                                                    TraitPath(`core::visual::Visualize`),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        4,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: Trait,
                                                                        expr_idx: 1,
                                                                    },
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 2,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TraitForTypeItemPath {
                                                                            impl_block: TraitForTypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::visual::Html`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Html`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::visual::Html`, `Extern`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `visualize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 3,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 3,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 4,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::ImplBlock(
            ImplBlockSynNodeDecl::Type(
                TypeImplBlockSynNodeDecl {
                    syn_node_path: TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                    impl_regional_token: ImplRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            1,
                        ),
                    },
                    template_parameter_decl_list: Ok(
                        None,
                    ),
                    self_ty_expr: SelfTypeObelisk {
                        expr: 1,
                    },
                    eol_colon: Ok(
                        EolRegionalToken::Colon(
                            EolColonRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                            },
                        ),
                    ),
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TypeImplBlock(
                                        TypeImplBlockSynNodePath {
                                            path: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExpr::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `ConcaveComponent`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
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
                            symbol_region: SynSymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: SelfType,
                                    expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MemoizedField(
                    TypeMemoizedFieldSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `norm`,
                                    item_kind: MemoizedField,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `norm`,
                                        item_kind: MemoizedField,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            colon_token: Ok(
                                Some(
                                    ColonRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            SynExpr::PrincipalEntityPath {
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
                                            PrincipalEntityPathExpr::Root {
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                3,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `hausdorff_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 2,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 3,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MemoizedField(
                    TypeMemoizedFieldSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `rel_norm`,
                                    item_kind: MemoizedField,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `rel_norm`,
                                        item_kind: MemoizedField,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            colon_token: Ok(
                                Some(
                                    ColonRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            SynExpr::PrincipalEntityPath {
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
                                            PrincipalEntityPathExpr::Root {
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                7,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 4,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ropd: 5,
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 6,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 7,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MemoizedField(
                    TypeMemoizedFieldSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `hausdorff_norm`,
                                    item_kind: MemoizedField,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `hausdorff_norm`,
                                        item_kind: MemoizedField,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            colon_token: Ok(
                                Some(
                                    ColonRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `hausdorff_norm`,
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
                                            SynExpr::PrincipalEntityPath {
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
                                            PrincipalEntityPathExpr::Root {
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                36,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `hausdorff_norm`,
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
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `hausdorff_norm`,
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
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `first`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 4,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 5,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        22,
                                                    ),
                                                ),
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `line_segment`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `curve_ls`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        34,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 10,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            36,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        38,
                                                    ),
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        40,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        41,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            42,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            44,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        45,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                },
                                                SynExpr::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        48,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        15,
                                                    ),
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        50,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 16,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        51,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            52,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 14,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        47,
                                                    ),
                                                    ropd: 15,
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 17,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        53,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            54,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        55,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        56,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 18,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        49,
                                                    ),
                                                    ropd: 19,
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        61,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 21,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        62,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            63,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        65,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        15,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 22,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        64,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 23,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        66,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 24,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        67,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            68,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `curve_ls`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        72,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        76,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 26,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        73,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `dist_to_point`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            74,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        75,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 27,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        77,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `point_dist`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        79,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `hausdorff_norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        81,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 29,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        80,
                                                    ),
                                                    ropd: 30,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `hausdorff_norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        83,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `point_dist`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        85,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 32,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        84,
                                                    ),
                                                    ropd: 33,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `hausdorff_norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        87,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        5..11,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 34,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                5,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                6..7,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                60,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 25,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                6,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                7..8,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                71,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 28,
                                                },
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                78,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            31,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        82,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                1,
                                                            ),
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 1,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                2,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 6,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            19,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                3,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                21,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 8,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                4,
                                                            ),
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                29,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 11,
                                                },
                                                SynStmt::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            39,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            48,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 15,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        14,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        19,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    frame_var_symbol_idx: 5,
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    57,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        2..5,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            86,
                                                        ),
                                                    },
                                                    result: 35,
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
                                                            ident: `hausdorff_norm`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `curve_start`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `curve_ls`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                20,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `dp_norm`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                28,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `point`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                59,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `point_dist`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                70,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    None,
                                                    None,
                                                    None,
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
                                                    SynPatternSymbol::Atom(
                                                        3,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        4,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        5,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        6,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `hausdorff_norm`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `curve_start`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `curve_ls`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `dp_norm`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `point`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `point_dist`,
                                                        6,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                    None,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    88,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `hausdorff_norm`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    88,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `curve_start`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            21,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    88,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `curve_ls`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            29,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    88,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `dp_norm`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            58,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    86,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 15,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            60,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    86,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `point`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            71,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    86,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `point_dist`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 11,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 25,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 28,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 34,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 35,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 36,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MemoizedField(
                    TypeMemoizedFieldSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `angle_change`,
                                    item_kind: MemoizedField,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `angle_change`,
                                        item_kind: MemoizedField,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            colon_token: Ok(
                                Some(
                                    ColonRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `angle_change`,
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
                                            SynExpr::PrincipalEntityPath {
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
                                            PrincipalEntityPathExpr::Root {
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                33,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `angle_change`,
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
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `angle_change`,
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
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 32,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        10,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 5,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 3,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 6,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        27,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 10,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                },
                                                SynExpr::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        12,
                                                    ),
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        37,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        38,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            39,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 11,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        34,
                                                    ),
                                                    ropd: 12,
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 14,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            41,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 15,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        36,
                                                    ),
                                                    ropd: 16,
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        48,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 18,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        49,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            50,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        52,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        12,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 19,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        51,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 20,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        53,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 21,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            55,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        56,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        57,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `dp0`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        60,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        64,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        66,
                                                    ),
                                                    Literal::Bool(
                                                        True,
                                                    ),
                                                ),
                                                SynExpr::CurrentSymbol {
                                                    ident: `angle_change`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        58,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 23,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        61,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_to`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        63,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 24,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    65,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 25,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        67,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 26,
                                                    opr: AssignClosed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        59,
                                                    ),
                                                    ropd: 27,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `dp0`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        68,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        70,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 29,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        69,
                                                    ),
                                                    ropd: 30,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `angle_change`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        72,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        4..8,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                3,
                                                            ),
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                47,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 22,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 28,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 31,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                1,
                                                            ),
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 1,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                2,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                9,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 8,
                                                },
                                                SynStmt::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            26,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            35,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 12,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        11,
                                                                    ),
                                                                    kind: LowerOpen,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        16,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    frame_var_symbol_idx: 3,
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    44,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            71,
                                                        ),
                                                    },
                                                    result: 32,
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
                                                            ident: `angle_change`,
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
                                                                        7,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `dp0`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                46,
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
                                                        `angle_change`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `dp0`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `dp`,
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    73,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `angle_change`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    73,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `dp0`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            45,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    71,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 12,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            47,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    71,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `dp`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 22,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 28,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 31,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 32,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 33,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MemoizedField(
                    TypeMemoizedFieldSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `bounding_box`,
                                    item_kind: MemoizedField,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `bounding_box`,
                                        item_kind: MemoizedField,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            colon_token: Ok(
                                Some(
                                    ColonRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `bounding_box`,
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
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BoundingBox`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                62,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `bounding_box`,
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
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `BoundingBox`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `bounding_box`,
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
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        4,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `first`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 3,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start_point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        18,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 6,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start_point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start_point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 10,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `start_point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            41,
                                                        ),
                                                    },
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        43,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 14,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        44,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 15,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        48,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        49,
                                                    ),
                                                },
                                                SynExpr::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        51,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        17,
                                                    ),
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        53,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 18,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            55,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 16,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        50,
                                                    ),
                                                    ropd: 17,
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 19,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        56,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            57,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        58,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        59,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 20,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        52,
                                                    ),
                                                    ropd: 21,
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        64,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 23,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        65,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            66,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        68,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        17,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 24,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        67,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 25,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        69,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 26,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        70,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            71,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `xmin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        74,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        78,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 29,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        79,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            80,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `xmin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        72,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 28,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        75,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `min`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            76,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        77,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 30,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        81,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 31,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        73,
                                                    ),
                                                    ropd: 32,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `xmax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        84,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        88,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 35,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        89,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            90,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `xmax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        82,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 34,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        85,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            86,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        87,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 36,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        91,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 37,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        83,
                                                    ),
                                                    ropd: 38,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ymin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        94,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        98,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 41,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        99,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            100,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ymin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        92,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 40,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        95,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `min`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        97,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 42,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        101,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 43,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        93,
                                                    ),
                                                    ropd: 44,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ymax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        104,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `point`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        108,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 47,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        109,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            110,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ymax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        102,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 46,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        105,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            106,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        107,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 48,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        111,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 49,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        103,
                                                    ),
                                                    ropd: 50,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `xmin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        117,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `xmax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        119,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 53,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        116,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 54,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    118,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 55,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        120,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ymin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        124,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ymax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        126,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 57,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        123,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 58,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    125,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 59,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        127,
                                                    ),
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 52,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        114,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 56,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    121,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 60,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    128,
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        129,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        6..13,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `BoundingBox`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                113,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `ClosedRange`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                115,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `ClosedRange`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                122,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            61,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                6,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                7..8,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                63,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 27,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 33,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 39,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 45,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 51,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                1,
                                                            ),
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 5,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                2,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 7,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            21,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                3,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                24,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 9,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                4,
                                                            ),
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                31,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 11,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            35,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            syn_pattern_root: SynPatternRoot(
                                                                5,
                                                            ),
                                                            variables: ArenaIdxRange(
                                                                5..6,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                38,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 13,
                                                },
                                                SynStmt::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            42,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            51,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 17,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        16,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        21,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    frame_var_symbol_idx: 6,
                                                    eol_colon: Ok(
                                                        EolRegionalToken::Colon(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    60,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: ArenaIdxRange(
                                                        1..6,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            112,
                                                        ),
                                                    },
                                                    result: 61,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `start_point`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        15,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `xmin`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        22,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `xmax`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                23,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        29,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `ymin`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                30,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        36,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `ymax`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                37,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `point`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                62,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    None,
                                                    Move,
                                                    Move,
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
                                                    SynPatternSymbol::Atom(
                                                        4,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        5,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        6,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `start_point`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `xmin`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `xmax`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `ymin`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `ymax`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `point`,
                                                        6,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    None,
                                                    Mut,
                                                    Mut,
                                                    Mut,
                                                    Mut,
                                                    None,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    130,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `start_point`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    130,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `xmin`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    130,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `xmax`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    130,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `ymin`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            38,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    130,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `ymax`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            61,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    112,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 17,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            63,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    112,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `point`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 5,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 7,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 9,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 11,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 13,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 27,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 33,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 39,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 45,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 51,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 61,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 62,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MemoizedField(
                    TypeMemoizedFieldSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `relative_bounding_box`,
                                    item_kind: MemoizedField,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `relative_bounding_box`,
                                        item_kind: MemoizedField,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            colon_token: Ok(
                                Some(
                                    ColonRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `relative_bounding_box`,
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
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `RelativeBoundingBox`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                7,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `relative_bounding_box`,
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
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `RelativeBoundingBox`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `relative_bounding_box`,
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
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `line_segment_sketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 5,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 6,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 7,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `line_segment`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `line_segment`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `line_segment`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `LineSegment`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                15,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `line_segment`,
                                                                            item_kind: MethodFn,
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `LineSegment`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `line_segment`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        3,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `first`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 4,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 5,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 6,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `clone`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        18,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `last`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 10,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 11,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `clone`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 1,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 7,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    17,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 13,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `LineSegment`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 14,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 14,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 15,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `start`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `start`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `start`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Point2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                7,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `start`,
                                                                            item_kind: MethodFn,
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Point2d`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `start`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `first`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 3,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 5,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `clone`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 6,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 7,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `end`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `end`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `end`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Point2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                7,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `end`,
                                                                            item_kind: MethodFn,
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Point2d`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `end`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `last`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 3,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 5,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `clone`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 6,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 7,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `displacement`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `displacement`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `displacement`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Vector2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                4,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `displacement`,
                                                                            item_kind: MethodFn,
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Vector2d`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `displacement`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `line_segment`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 3,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 3,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 4,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `start_tangent`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `start_tangent`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `start_tangent`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Vector2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                6,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `start_tangent`,
                                                                            item_kind: MethodFn,
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Vector2d`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `start_tangent`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `first`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 3,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 5,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 5,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 6,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `end_tangent`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `end_tangent`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameters {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConcaveComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: SynSymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `end_tangent`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Vector2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                6,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExpr::PrincipalEntityPath {
                                                                            path_expr_idx: 1,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        PrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                                IdentRegionalToken {
                                                                                    ident: `ConcaveComponent`,
                                                                                    regional_token_idx: RegionalTokenIdx(
                                                                                        2,
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
                                                                symbol_region: SynSymbolRegion {
                                                                    inherited_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `end_tangent`,
                                                                            item_kind: MethodFn,
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Vector2d`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `end_tangent`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        1,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `last`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 3,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 4,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 5,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 5,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 6,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
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