Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            24,
                                        ),
                                    ),
                                    ty: 0,
                                },
                                SpecificParameterObelisk::Regular {
                                    pattern: 1,
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            28,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonObelisk {
                                    expr: 2,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::basic::bool`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `LineSegmentSketch`,
                                                        token_idx: TokenIdx(
                                                            25,
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `bool`,
                                                        token_idx: TokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::basic::bool`, `Extern`),
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
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `line_segment_sketch`,
                                                        token_idx: TokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `index`,
                                                        token_idx: TokenIdx(
                                                            27,
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
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    0,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `line_segment_sketch`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `index`,
                                                    1,
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        24,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `line_segment_sketch`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        28,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `index`,
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
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 1,
                                                    ty_expr_idx: 1,
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
                                            expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 2,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            93,
                        ),
                        syn_expr_region: SynExprRegion {
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
                                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::basic::bool`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `LineSegmentSketch`,
                                                                token_idx: TokenIdx(
                                                                    25,
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `i32`,
                                                                token_idx: TokenIdx(
                                                                    29,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `bool`,
                                                                token_idx: TokenIdx(
                                                                    32,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::basic::bool`, `Extern`),
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
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `line_segment_sketch`,
                                                                token_idx: TokenIdx(
                                                                    23,
                                                                ),
                                                            },
                                                        },
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `index`,
                                                                token_idx: TokenIdx(
                                                                    27,
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
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        SynPatternSymbol::Atom(
                                                            0,
                                                        ),
                                                        SynPatternSymbol::Atom(
                                                            1,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `line_segment_sketch`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `index`,
                                                            1,
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
                                            symbol_region: SynSymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                24,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `line_segment_sketch`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                28,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `index`,
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
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 0,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 1,
                                                            ty_expr_idx: 1,
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
                                                    expr_idx: 0,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
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
                                expr_arena: Arena {
                                    data: [
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                37,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                38,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                40,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                42,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                43,
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                47,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                48,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    49,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                51,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                53,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                52,
                                            ),
                                            ropd: 6,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 4,
                                            lbox_token_idx: TokenIdx(
                                                50,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 7,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                54,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 8,
                                            dot_token_idx: TokenIdx(
                                                55,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    56,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                57,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                58,
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                62,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                63,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    64,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                67,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                69,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                68,
                                            ),
                                            ropd: 13,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                66,
                                            ),
                                            item: 14,
                                            rpar_token_idx: TokenIdx(
                                                70,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                72,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 15,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                71,
                                            ),
                                            ropd: 16,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 11,
                                            lbox_token_idx: TokenIdx(
                                                65,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 17,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 18,
                                            dot_token_idx: TokenIdx(
                                                74,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    75,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                76,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                77,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_displacement`,
                                            token_idx: TokenIdx(
                                                81,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                85,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 20,
                                            dot_token_idx: TokenIdx(
                                                82,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rotation_direction_to`,
                                                token_idx: TokenIdx(
                                                    83,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                84,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 21,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                86,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                88,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                90,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 23,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                89,
                                            ),
                                            ropd: 24,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                97,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 33,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                96,
                                            ),
                                            opd: 26,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                101,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 28,
                                            dot_token_idx: TokenIdx(
                                                102,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                106,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                108,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 30,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                107,
                                            ),
                                            ropd: 31,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                105,
                                            ),
                                            item: 32,
                                            rpar_token_idx: TokenIdx(
                                                109,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                111,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 33,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                110,
                                            ),
                                            ropd: 34,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 29,
                                            lbox_token_idx: TokenIdx(
                                                104,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 35,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                112,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 36,
                                            dot_token_idx: TokenIdx(
                                                113,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                116,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 38,
                                            dot_token_idx: TokenIdx(
                                                117,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                119,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                120,
                                            ),
                                        },
                                        SynExpr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident: `i1`,
                                            frame_var_symbol_idx: 6,
                                            current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                40,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                124,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 39,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                121,
                                            ),
                                            ropd: 40,
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 41,
                                            dot_token_idx: TokenIdx(
                                                125,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                127,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                128,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 42,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                123,
                                            ),
                                            ropd: 43,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                133,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 45,
                                            dot_token_idx: TokenIdx(
                                                134,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `contour`,
                                                token_idx: TokenIdx(
                                                    135,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                139,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 47,
                                            dot_token_idx: TokenIdx(
                                                140,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                142,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                143,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `i1`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                40,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 46,
                                            dot_token_idx: TokenIdx(
                                                136,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    137,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                138,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 48,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            144,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 49,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                146,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                157,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 52,
                                            dot_token_idx: TokenIdx(
                                                154,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `cross`,
                                                token_idx: TokenIdx(
                                                    155,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                156,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 53,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                158,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                147,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 51,
                                            dot_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                152,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 54,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                159,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 55,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                148,
                                            ),
                                            ropd: 56,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                165,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 34,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                164,
                                            ),
                                            opd: 58,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                169,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 60,
                                            dot_token_idx: TokenIdx(
                                                170,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    171,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                173,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                175,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 62,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                174,
                                            ),
                                            ropd: 63,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 61,
                                            lbox_token_idx: TokenIdx(
                                                172,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 64,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                176,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 65,
                                            dot_token_idx: TokenIdx(
                                                177,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    178,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_interval`,
                                            token_idx: TokenIdx(
                                                180,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 67,
                                            dot_token_idx: TokenIdx(
                                                181,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    182,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                183,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                184,
                                            ),
                                        },
                                        SynExpr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                186,
                                            ),
                                            ident: `i2`,
                                            frame_var_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                69,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_interval`,
                                            token_idx: TokenIdx(
                                                188,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 68,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                185,
                                            ),
                                            ropd: 69,
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 70,
                                            dot_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    190,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                191,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                192,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 71,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                187,
                                            ),
                                            ropd: 72,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                197,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 74,
                                            dot_token_idx: TokenIdx(
                                                198,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `contour`,
                                                token_idx: TokenIdx(
                                                    199,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                203,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 76,
                                            dot_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    205,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                206,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                207,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `i2`,
                                            token_idx: TokenIdx(
                                                209,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                69,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 75,
                                            dot_token_idx: TokenIdx(
                                                200,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    201,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                202,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 77,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            208,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 78,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                213,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                217,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                221,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 81,
                                            dot_token_idx: TokenIdx(
                                                218,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `cross`,
                                                token_idx: TokenIdx(
                                                    219,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                220,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 82,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                222,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                211,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 80,
                                            dot_token_idx: TokenIdx(
                                                214,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    215,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                216,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 83,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                223,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 84,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ropd: 85,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                225,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                227,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 87,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                226,
                                            ),
                                            ropd: 88,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                231,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                233,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 90,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                232,
                                            ),
                                            ropd: 91,
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                12..17,
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
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 6,
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
                                                EqToken(
                                                    TokenIdx(
                                                        132,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 50,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 57,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    194,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 9,
                                                    variables: ArenaIdxRange(
                                                        11..12,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        196,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 79,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 86,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    92,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 4,
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
                                                EqToken(
                                                    TokenIdx(
                                                        95,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 27,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    98,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 5,
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
                                                EqToken(
                                                    TokenIdx(
                                                        100,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 37,
                                        },
                                        SynStmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    115,
                                                ),
                                            },
                                            particulars: SynForBetweenParticulars {
                                                for_between_loop_var_token_idx: TokenIdx(
                                                    122,
                                                ),
                                                for_between_loop_var_ident: `i1`,
                                                for_between_loop_var_expr_idx: 40,
                                                range: SynForBetweenRange {
                                                    initial_boundary: SynForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            39,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: SynForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            43,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            frame_var_symbol_idx: 6,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            129,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..2,
                                                ),
                                            ),
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    160,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 7,
                                                    variables: ArenaIdxRange(
                                                        8..9,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        163,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 59,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    166,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 8,
                                                    variables: ArenaIdxRange(
                                                        9..10,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        168,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 66,
                                        },
                                        SynStmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    179,
                                                ),
                                            },
                                            particulars: SynForBetweenParticulars {
                                                for_between_loop_var_token_idx: TokenIdx(
                                                    186,
                                                ),
                                                for_between_loop_var_ident: `i2`,
                                                for_between_loop_var_expr_idx: 69,
                                                range: SynForBetweenRange {
                                                    initial_boundary: SynForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            68,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: SynForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            72,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            frame_var_symbol_idx: 10,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            193,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    2..4,
                                                ),
                                            ),
                                        },
                                        SynStmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    224,
                                                ),
                                            },
                                            result: 89,
                                        },
                                        SynStmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    230,
                                                ),
                                            },
                                            result: 92,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        36,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 2,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    44,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 1,
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
                                                EqToken(
                                                    TokenIdx(
                                                        46,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 9,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    59,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 2,
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
                                                EqToken(
                                                    TokenIdx(
                                                        61,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 19,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    78,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 3,
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
                                                EqToken(
                                                    TokenIdx(
                                                        80,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 22,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                },
                                                condition: Ok(
                                                    25,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                91,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        4..11,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                SynElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            228,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    229,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    stmts: Ok(
                                                        ArenaIdxRange(
                                                            11..12,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        35,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `current_displacement`,
                                                    token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `previous_displacement`,
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                93,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `previous_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                161,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `current_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        162,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `current_interval`,
                                                    token_idx: TokenIdx(
                                                        167,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            Move,
                                            None,
                                            None,
                                            Move,
                                            None,
                                            None,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            SynPatternSymbol::Atom(
                                                0,
                                            ),
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
                                            SynPatternSymbol::Atom(
                                                7,
                                            ),
                                            SynPatternSymbol::Atom(
                                                8,
                                            ),
                                            SynPatternSymbol::Atom(
                                                9,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `L`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `current_displacement`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `previous_displacement`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `is_rotation_counterclockwise_result`,
                                                3,
                                            ),
                                        ],
                                        [
                                            (
                                                `previous_raw_cross`,
                                                4,
                                            ),
                                        ],
                                        [
                                            (
                                                `previous_interval`,
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                `displacement`,
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                `current_raw_cross`,
                                                7,
                                            ),
                                        ],
                                        [
                                            (
                                                `current_interval`,
                                                8,
                                            ),
                                        ],
                                        [
                                            (
                                                `displacement`,
                                                9,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            Mut,
                                            None,
                                            None,
                                            Mut,
                                            None,
                                            None,
                                        ],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `index`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    36,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            234,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `L`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    46,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            234,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `current_displacement`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    61,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            234,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `previous_displacement`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    80,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            234,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    95,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            228,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `previous_raw_cross`,
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    100,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            228,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `previous_interval`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    130,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            160,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::FrameVariable {
                                                    ident: `i1`,
                                                    expr_idx: 40,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    132,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            160,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `displacement`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    163,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            228,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `current_raw_cross`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    168,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            228,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `current_interval`,
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    194,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::FrameVariable {
                                                    ident: `i2`,
                                                    expr_idx: 69,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    196,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `displacement`,
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            FrameVariable,
                                            ArenaIdxRange(
                                                6..7,
                                            ),
                                        ),
                                        (
                                            FrameVariable,
                                            ArenaIdxRange(
                                                10..11,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 2,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 9,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 19,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 22,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 27,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 37,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 50,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 57,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 59,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 66,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 79,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 86,
                                    },
                                    SynExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 89,
                                    },
                                    SynExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 92,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 93,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)