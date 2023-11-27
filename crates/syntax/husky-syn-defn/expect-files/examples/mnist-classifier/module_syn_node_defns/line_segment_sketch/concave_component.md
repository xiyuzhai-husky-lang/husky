[
    ItemSynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Type(
            TypeSynNodeDefn::PropsStruct(
                PropsStructTypeSynNodeDefn {
                    syn_node_path: TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: PropsStructTypeSynNodeDecl {
                        syn_node_path: TypeSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Type(
                                        TypeSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                    PropsFieldSyndicate {
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
                                        ty_syn_expr_idx: 2,
                                        initialization: None,
                                        variable: 1,
                                    },
                                    PropsFieldSyndicate {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentRegionalToken {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 374,
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
                                        ty_syn_expr_idx: 6,
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
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Type(
                                            TypeSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Type(
                                                            TypeSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
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
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::Prefix {
                                            opr: Tilde,
                                            opr_regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                            opd: 3,
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 4,
                                            argument_expr_idx: 5,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                symbol_region: SynSymbolRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: RegionalTokenIdx(
                                                    9,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::FieldVariable {
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
                                                data: CurrentSynSymbolData::FieldVariable {
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
                                                                value: 374,
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
                                syn_pattern_expr_roots: [],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::PropsStructFieldType {
                                            ident_token: IdentRegionalToken {
                                                ident: `line_segment_sketch`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                        },
                                        syn_expr_idx: 2,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::PropsStructFieldType {
                                            ident_token: IdentRegionalToken {
                                                ident: `strokes`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                        },
                                        syn_expr_idx: 6,
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
    ItemSynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::FunctionFn(
                FnSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: FnSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_obelisk_list: Ok(
                            None,
                        ),
                        parenate_parameter_obelisk_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 5,
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
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Fugitive(
                                                            FugitiveSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::Prefix {
                                            opr: Tilde,
                                            opr_regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                            opd: 1,
                                        },
                                        SynExprData::List {
                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                11,
                                            ),
                                            items: [],
                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 3,
                                            argument_expr_idx: 4,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                ty_expr_idx: 2,
                                            },
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 1,
                                    },
                                ],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 2,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ReturnType,
                                        syn_expr_idx: 5,
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
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::MajorItem(
                                                        MajorItemSynNodePath::Fugitive(
                                                            FugitiveSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::MajorItem(
                                                                        MajorItemSynNodePathData::Fugitive(
                                                                            FugitiveSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::Prefix {
                                                            opr: Tilde,
                                                            opr_regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                            opd: 1,
                                                        },
                                                        SynExprData::List {
                                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                                11,
                                                            ),
                                                            items: [],
                                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 1,
                                                                },
                                                                ty_expr_idx: 2,
                                                            },
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                ],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 2,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ReturnType,
                                                        syn_expr_idx: 5,
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
                                    path: SynNodeRegionPath::Defn(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    6,
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
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `line_segment_sketch`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    20,
                                                ),
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    30,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::CurrentSynSymbol {
                                                ident: `L`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    35,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Prefix {
                                                opr: Minus,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    34,
                                                ),
                                                opd: 10,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `line_segment_sketch`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    40,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    42,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 13,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    39,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 14,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                41,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 15,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    43,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 11,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    33,
                                                ),
                                                ropd: 12,
                                            },
                                            SynExprData::Prefix {
                                                opr: Not,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    37,
                                                ),
                                                opd: 16,
                                            },
                                            SynExprData::Binary {
                                                lopd: 17,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    36,
                                                ),
                                                ropd: 18,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    45,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Suffix {
                                                opd: 20,
                                                opr: Decr,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    46,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    50,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `ccv_start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    54,
                                                ),
                                                current_syn_symbol_idx: 5,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `L`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    56,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    52,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 23,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    55,
                                                ),
                                                ropd: 24,
                                            },
                                            SynExprData::Binary {
                                                lopd: 25,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    53,
                                                ),
                                                ropd: 26,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    61,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `L`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    63,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    59,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 28,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    62,
                                                ),
                                                ropd: 29,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `line_segment_sketch`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    68,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    70,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 32,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    67,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 33,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                69,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 34,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    71,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 30,
                                                opr: Comparison(
                                                    Leq,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    60,
                                                ),
                                                ropd: 31,
                                            },
                                            SynExprData::Prefix {
                                                opr: Not,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    65,
                                                ),
                                                opd: 35,
                                            },
                                            SynExprData::Binary {
                                                lopd: 36,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    64,
                                                ),
                                                ropd: 37,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    73,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::Suffix {
                                                opd: 39,
                                                opr: Incr,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    74,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    78,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    80,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::CurrentSynSymbol {
                                                ident: `end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    76,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 41,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    79,
                                                ),
                                                ropd: 42,
                                            },
                                            SynExprData::Binary {
                                                lopd: 43,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    77,
                                                ),
                                                ropd: 44,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `concave_components`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    82,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `line_segment_sketch`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    88,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `line_segment_sketch`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    90,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    96,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    98,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    95,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 51,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                97,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 52,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    99,
                                                ),
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 47,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    87,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 48,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                89,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 53,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    100,
                                                ),
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    85,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 54,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    101,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    102,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    104,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 56,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    103,
                                                ),
                                                ropd: 57,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    107,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    109,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::CurrentSynSymbol {
                                                ident: `end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    105,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 59,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    108,
                                                ),
                                                ropd: 60,
                                            },
                                            SynExprData::Binary {
                                                lopd: 61,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    106,
                                                ),
                                                ropd: 62,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `concave_components`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    111,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    8..16,
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
                                            SynPrincipalEntityPathExpr::Root {
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
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
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
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
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
                                            SynStmtData::Eval {
                                                expr_idx: 21,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 40,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 55,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::While {
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
                                            SynStmtData::IfElse {
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
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                81,
                                                            ),
                                                        },
                                                    ),
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                                elif_branches: [],
                                                else_branch: None,
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 58,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 63,
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
                                                            3,
                                                        ),
                                                    },
                                                ),
                                                assign_token: Ok(
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 4,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        11,
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
                                                            None,
                                                        ),
                                                        ty: None,
                                                    },
                                                ),
                                                assign_token: Ok(
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 7,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 3,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 8,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 4,
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
                                                            29,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 9,
                                            },
                                            SynStmtData::While {
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
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        47,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 5,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            49,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 22,
                                            },
                                            SynStmtData::While {
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
                                            SynStmtData::Return {
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                                                112,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    pattern: LetSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
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
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 4,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 5,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtType,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 7,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 8,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 9,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 21,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 22,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 40,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 55,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 58,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 63,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnExpr,
                                            syn_expr_idx: 64,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 65,
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
                                            3,
                                        ),
                                        (
                                            4,
                                            4,
                                        ),
                                        (
                                            5,
                                            5,
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
    ItemSynNodeDefn::ImplBlock(
        ImplBlockSynNodeDecl::TraitForType(
            TraitForTypeImplBlockSynNodeDecl {
                syn_node_path: TraitForTypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                TraitForTypeImplBlockSynNodePathData {
                                    path: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                },
                            ),
                        ),
                    },
                ),
                impl_regional_token: ImplRegionalToken {
                    regional_token_idx: RegionalTokenIdx(
                        1,
                    ),
                },
                template_parameter_decl_list: Ok(
                    None,
                ),
                trai_expr: TraitSyndicate {
                    expr: 1,
                },
                for_token: ConnectionForRegionalToken {
                    regional_token_idx: RegionalTokenIdx(
                        3,
                    ),
                },
                self_ty_decl: PathLeadingExpr(
                    SelfTypeSyndicate {
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
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        expr_arena: Arena {
                            data: [
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::visual::Visualize`),
                                            ),
                                        ),
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
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
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
                                SynPrincipalEntityPathExpr::Root {
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
                                kind: SynExprRootKind::Trait,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::SelfType,
                                syn_expr_idx: 2,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TraitForTypeItem(
            TraitForTypeItemSynNodeDefn::MethodFn(
                TraitForTypeMethodFnSynNodeDefn {
                    syn_node_path: TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitForTypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 391,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TraitForTypeMethodFnSynNodeDecl {
                        syn_node_path: TraitForTypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TraitForTypeItem(
                                        TraitForTypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitForTypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 391,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        parenate_parameter_decl_list: Ok(
                            ParenateParameterSyndicateList {
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
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
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                                    ty_sketch: TypeSketch::Path(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::visual::Visualize`),
                                                                ),
                                                            ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                                    SynPrincipalEntityPathExpr::Root {
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
                                                    kind: SynExprRootKind::Trait,
                                                    syn_expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 2,
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
                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                            TraitForTypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TraitForTypeItem(
                                                            TraitForTypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TraitForTypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 391,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                                                    ty_sketch: TypeSketch::Path(
                                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            expr_arena: Arena {
                                                                data: [
                                                                    SynExprData::PrincipalEntityPath {
                                                                        path_expr_idx: 1,
                                                                        opt_path: Some(
                                                                            PrincipalEntityPath::MajorItem(
                                                                                MajorItemPath::Trait(
                                                                                    TraitPath(`core::visual::Visualize`),
                                                                                ),
                                                                            ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                                    kind: SynExprRootKind::Trait,
                                                                    syn_expr_idx: 1,
                                                                },
                                                                SynExprRoot {
                                                                    kind: SynExprRootKind::SelfType,
                                                                    syn_expr_idx: 2,
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
                                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                                            TraitForTypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                            TraitForTypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TraitForTypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 391,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TraitForTypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 391,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::ImplBlock(
        ImplBlockSynNodeDecl::Type(
            TypeImplBlockSynNodeDecl {
                syn_node_path: TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(
                                        ItemPathId(
                                            Id {
                                                value: 346,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                impl_regional_token: ImplRegionalToken {
                    regional_token_idx: RegionalTokenIdx(
                        1,
                    ),
                },
                template_parameter_decl_list: Ok(
                    None,
                ),
                self_ty_expr: SelfTypeSyndicate {
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
                        path: SynNodeRegionPath::Decl(
                            ItemSynNodePath::ImplBlock(
                                ImplBlockSynNodePath::TypeImplBlock(
                                    TypeImplBlockSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::ImplBlock(
                                                ImplBlockSynNodePathData::TypeImplBlock(
                                                    TypeImplBlockSynNodePathData {
                                                        path: TypeImplBlockPath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 346,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
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
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MemoizedField(
                TypeMemoizedFieldSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 392,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 392,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                ReturnTypeBeforeEqSyndicate {
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 392,
                                                                            },
                                                                        ),
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 392,
                                                                                            },
                                                                                        ),
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 392,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 3,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MemoizedField(
                TypeMemoizedFieldSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 393,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 393,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                ReturnTypeBeforeEqSyndicate {
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 393,
                                                                            },
                                                                        ),
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 393,
                                                                                            },
                                                                                        ),
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 393,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    5,
                                                ),
                                            ),
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 4,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                ropd: 5,
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 7,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MemoizedField(
                TypeMemoizedFieldSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 394,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 394,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                ReturnTypeBeforeEqSyndicate {
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 394,
                                                                            },
                                                                        ),
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 394,
                                                                                            },
                                                                                        ),
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 394,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    5,
                                                ),
                                                LiteralData::Float(
                                                    Unspecified(
                                                        UnspecifiedFloatLiteral(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 4,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    22,
                                                ),
                                            ),
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    26,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `curve_ls`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    30,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    33,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    34,
                                                ),
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    37,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    38,
                                                ),
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    40,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    45,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    46,
                                                ),
                                            },
                                            SynExprData::FrameVarDecl {
                                                regional_token_idx: RegionalTokenIdx(
                                                    48,
                                                ),
                                                ident: `i`,
                                                frame_var_symbol_idx: 5,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                    15,
                                                ),
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    50,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::Binary {
                                                lopd: 14,
                                                opr: Comparison(
                                                    Leq,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    47,
                                                ),
                                                ropd: 15,
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    55,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    56,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 18,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    49,
                                                ),
                                                ropd: 19,
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    61,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `i`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    65,
                                                ),
                                                current_syn_symbol_idx: 5,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                    15,
                                                ),
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 22,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    64,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 23,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    66,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `curve_ls`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    72,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    76,
                                                ),
                                                current_syn_symbol_idx: 6,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    75,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 27,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    77,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `point_dist`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    79,
                                                ),
                                                current_syn_symbol_idx: 7,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `hausdorff_norm`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    81,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 29,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    80,
                                                ),
                                                ropd: 30,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `hausdorff_norm`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    83,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `point_dist`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    85,
                                                ),
                                                current_syn_symbol_idx: 7,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 32,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    84,
                                                ),
                                                ropd: 33,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `hausdorff_norm`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    87,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
                                                expr_idx: 34,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        58,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 5,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            60,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 25,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        69,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 6,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            71,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 28,
                                            },
                                            SynStmtData::IfElse {
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
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                82,
                                                            ),
                                                        },
                                                    ),
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                elif_branches: [],
                                                else_branch: None,
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
                                                initial_value: 1,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
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
                                                            None,
                                                        ),
                                                        ty: None,
                                                    },
                                                ),
                                                assign_token: Ok(
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 6,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 3,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            21,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 8,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        27,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 4,
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
                                                            29,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 11,
                                            },
                                            SynStmtData::ForBetween {
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
                                                for_loop_var_symbol_idx: 5,
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
                                            SynStmtData::Return {
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
                                                                88,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LoopVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                LoopVariable,
                                                ArenaIdxRange(
                                                    5..6,
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
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 4,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 5,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 6,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 8,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 11,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 25,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 28,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 34,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnExpr,
                                            syn_expr_idx: 35,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 36,
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
                                            3,
                                        ),
                                        (
                                            4,
                                            4,
                                        ),
                                        (
                                            5,
                                            6,
                                        ),
                                        (
                                            6,
                                            7,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MemoizedField(
                TypeMemoizedFieldSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 395,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 395,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                ReturnTypeBeforeEqSyndicate {
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 395,
                                                                            },
                                                                        ),
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 395,
                                                                                            },
                                                                                        ),
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 395,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    5,
                                                ),
                                                LiteralData::Float(
                                                    Unspecified(
                                                        UnspecifiedFloatLiteral(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    10,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    14,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    20,
                                                ),
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 3,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 6,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    24,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    27,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    33,
                                                ),
                                            },
                                            SynExprData::FrameVarDecl {
                                                regional_token_idx: RegionalTokenIdx(
                                                    35,
                                                ),
                                                ident: `i`,
                                                frame_var_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                    12,
                                                ),
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    37,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::Binary {
                                                lopd: 11,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    34,
                                                ),
                                                ropd: 12,
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    42,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    43,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 15,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    36,
                                                ),
                                                ropd: 16,
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    48,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `i`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    52,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                    12,
                                                ),
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 19,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    51,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 20,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    53,
                                                ),
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    56,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    57,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `dp0`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    60,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `dp`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    64,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    66,
                                                ),
                                                LiteralData::Bool(
                                                    True,
                                                ),
                                            ),
                                            SynExprData::CurrentSynSymbol {
                                                ident: `angle_change`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    58,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    63,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 24,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                65,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 25,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    67,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 26,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    59,
                                                ),
                                                ropd: 27,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `dp0`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    68,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `dp`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    70,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 29,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    69,
                                                ),
                                                ropd: 30,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `angle_change`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    72,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        45,
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
                                                            47,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 22,
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 28,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 31,
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
                                                initial_value: 1,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
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
                                                            None,
                                                        ),
                                                        ty: None,
                                                    },
                                                ),
                                                assign_token: Ok(
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 8,
                                            },
                                            SynStmtData::ForBetween {
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
                                                for_loop_var_symbol_idx: 3,
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
                                            SynStmtData::Return {
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
                                                                73,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LoopVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 8,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 22,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 28,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 31,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnExpr,
                                            syn_expr_idx: 32,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 33,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MemoizedField(
                TypeMemoizedFieldSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 396,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 396,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                ReturnTypeBeforeEqSyndicate {
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 396,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 396,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 396,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 3,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start_point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start_point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start_point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `start_point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    39,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    43,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    48,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    49,
                                                ),
                                            },
                                            SynExprData::FrameVarDecl {
                                                regional_token_idx: RegionalTokenIdx(
                                                    51,
                                                ),
                                                ident: `i`,
                                                frame_var_symbol_idx: 6,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                    17,
                                                ),
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    53,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::Binary {
                                                lopd: 16,
                                                opr: Comparison(
                                                    Leq,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    50,
                                                ),
                                                ropd: 17,
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    58,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    59,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 20,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    52,
                                                ),
                                                ropd: 21,
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    64,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `i`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    68,
                                                ),
                                                current_syn_symbol_idx: 6,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                                    17,
                                                ),
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 24,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    67,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 25,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    69,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `xmin`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    74,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    78,
                                                ),
                                                current_syn_symbol_idx: 7,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `xmin`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    72,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    77,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 30,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    81,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 31,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    73,
                                                ),
                                                ropd: 32,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `xmax`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    84,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    88,
                                                ),
                                                current_syn_symbol_idx: 7,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `xmax`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    82,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    87,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 36,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    91,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 37,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    83,
                                                ),
                                                ropd: 38,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `ymin`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    94,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    98,
                                                ),
                                                current_syn_symbol_idx: 7,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `ymin`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    92,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    97,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 42,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    101,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 43,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    93,
                                                ),
                                                ropd: 44,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `ymax`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    104,
                                                ),
                                                current_syn_symbol_idx: 5,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `point`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    108,
                                                ),
                                                current_syn_symbol_idx: 7,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::CurrentSynSymbol {
                                                ident: `ymax`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    102,
                                                ),
                                                current_syn_symbol_idx: 5,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    107,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 48,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    111,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 49,
                                                opr: Assign,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    103,
                                                ),
                                                ropd: 50,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `xmin`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    117,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `xmax`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    119,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 53,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    116,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 54,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                118,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 55,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    120,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `ymin`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    124,
                                                ),
                                                current_syn_symbol_idx: 4,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `ymax`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    126,
                                                ),
                                                current_syn_symbol_idx: 5,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 57,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    123,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 58,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                125,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 59,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    127,
                                                ),
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 52,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    114,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 56,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                121,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 60,
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
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    6..13,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
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
                                            SynPrincipalEntityPathExpr::Root {
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
                                            SynPrincipalEntityPathExpr::Root {
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
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        61,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 6,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            63,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 27,
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 33,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 39,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 45,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 51,
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
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 5,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        14,
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
                                                            None,
                                                        ),
                                                        ty: None,
                                                    },
                                                ),
                                                assign_token: Ok(
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 7,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 3,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 9,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 4,
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
                                                            31,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 11,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 5,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            38,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 13,
                                            },
                                            SynStmtData::ForBetween {
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
                                                for_loop_var_symbol_idx: 6,
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
                                            SynStmtData::Return {
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                    data: CurrentSynSymbolData::LoopVariable {
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
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                LoopVariable,
                                                ArenaIdxRange(
                                                    6..7,
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
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 4,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 5,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 6,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 7,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 9,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 11,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 13,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 27,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 33,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 39,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 45,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 51,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnExpr,
                                            syn_expr_idx: 61,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 62,
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
                                            3,
                                        ),
                                        (
                                            4,
                                            4,
                                        ),
                                        (
                                            5,
                                            5,
                                        ),
                                        (
                                            6,
                                            7,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MemoizedField(
                TypeMemoizedFieldSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 397,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMemoizedFieldSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 397,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                ReturnTypeBeforeEqSyndicate {
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 397,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 397,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 397,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::Field {
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
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 5,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 7,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 398,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 398,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 398,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 398,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 398,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    3,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 4,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    18,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    23,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    24,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 10,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    30,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 1,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 7,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 13,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 14,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 15,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 399,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 399,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 399,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 399,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 399,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 3,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 7,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 400,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 400,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 400,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 400,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 400,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 3,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 7,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 401,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 401,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 401,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 401,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 401,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 402,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 402,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 402,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 402,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 402,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 3,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 6,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 403,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::AssociatedItem(
                                    AssociatedItemSynNodePathData::TypeItem(
                                        TypeItemSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 403,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
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
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath(
                                                            ItemSynNodePathId {
                                                                data: ItemSynNodePathData::ImplBlock(
                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                        TypeImplBlockSynNodePathData {
                                                                            path: TypeImplBlockPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 346,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
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
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
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
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::AssociatedItem(
                                                        AssociatedItemSynNodePathData::TypeItem(
                                                            TypeItemSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TypeItemPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 403,
                                                                            },
                                                                        ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
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
                                        SynPrincipalEntityPathExpr::Root {
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
                                                            path: SynNodeRegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                                        TypeImplBlockSynNodePath(
                                                                            ItemSynNodePathId {
                                                                                data: ItemSynNodePathData::ImplBlock(
                                                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                                                        TypeImplBlockSynNodePathData {
                                                                                            path: TypeImplBlockPath(
                                                                                                ItemPathId(
                                                                                                    Id {
                                                                                                        value: 346,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                                ],
                                                            },
                                                            principal_item_path_expr_arena: Arena {
                                                                data: [
                                                                    SynPrincipalEntityPathExpr::Root {
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
                                                        AssociatedItemSynNodePath::TypeItem(
                                                            TypeItemSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::AssociatedItem(
                                                                        AssociatedItemSynNodePathData::TypeItem(
                                                                            TypeItemSynNodePathData {
                                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                    path: TypeItemPath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 403,
                                                                                            },
                                                                                        ),
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
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
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
                                                        SynPrincipalEntityPathExpr::Root {
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
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 403,
                                                                                },
                                                                            ),
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfValue(
                                                RegionalTokenIdx(
                                                    1,
                                                ),
                                            ),
                                            SynExprData::Field {
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
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 3,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExprData::MethodApplicationOrCall {
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
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Eval {
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
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 6,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]