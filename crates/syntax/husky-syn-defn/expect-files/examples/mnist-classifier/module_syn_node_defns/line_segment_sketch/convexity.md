Ok(
    [
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 22,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenic_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            21,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenic_parameters: [
                                        SpecificParameterDecl::Regular {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    23,
                                                ),
                                            ),
                                            ty: 0,
                                        },
                                        SpecificParameterDecl::Regular {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    27,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                25,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            29,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            30,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            32,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
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
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
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
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `bool`,
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternSynExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `line_segment_sketch`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                                PatternSynExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `index`,
                                                        token_idx: TokenIdx(
                                                            26,
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
                                                PatternSynSymbol::Atom(
                                                    0,
                                                ),
                                                PatternSynSymbol::Atom(
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
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenicRegularParameter {
                                                        ident: `line_segment_sketch`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        27,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
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
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
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
                                                                    24,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `i32`,
                                                                token_idx: TokenIdx(
                                                                    28,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `bool`,
                                                                token_idx: TokenIdx(
                                                                    31,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::basic::bool`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternSynExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `line_segment_sketch`,
                                                                token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                            },
                                                        },
                                                        PatternSynExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `index`,
                                                                token_idx: TokenIdx(
                                                                    26,
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
                                                        PatternSynSymbol::Atom(
                                                            0,
                                                        ),
                                                        PatternSynSymbol::Atom(
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
                                                                23,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenicRegularParameter {
                                                                ident: `line_segment_sketch`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                27,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
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
                                                36,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                37,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    38,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                39,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    40,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                41,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                42,
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                46,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                47,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    48,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                50,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                52,
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
                                                51,
                                            ),
                                            ropd: 6,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 4,
                                            lbox_token_idx: TokenIdx(
                                                49,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 7,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                53,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 8,
                                            dot_token_idx: TokenIdx(
                                                54,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    55,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                56,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                57,
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                61,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                62,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    63,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                68,
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
                                                67,
                                            ),
                                            ropd: 13,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                65,
                                            ),
                                            item: 14,
                                            rpar_token_idx: TokenIdx(
                                                69,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                71,
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
                                                70,
                                            ),
                                            ropd: 16,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 11,
                                            lbox_token_idx: TokenIdx(
                                                64,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 17,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                72,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 18,
                                            dot_token_idx: TokenIdx(
                                                73,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    74,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                75,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                76,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_displacement`,
                                            token_idx: TokenIdx(
                                                80,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 20,
                                            dot_token_idx: TokenIdx(
                                                81,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rotation_direction_to`,
                                                token_idx: TokenIdx(
                                                    82,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 21,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                85,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                87,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                89,
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
                                                88,
                                            ),
                                            ropd: 24,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                96,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                95,
                                            ),
                                            opd: 26,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                100,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 28,
                                            dot_token_idx: TokenIdx(
                                                101,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    102,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                105,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                107,
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
                                                106,
                                            ),
                                            ropd: 31,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                104,
                                            ),
                                            item: 32,
                                            rpar_token_idx: TokenIdx(
                                                108,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                110,
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
                                                109,
                                            ),
                                            ropd: 34,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 29,
                                            lbox_token_idx: TokenIdx(
                                                103,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 35,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                111,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 36,
                                            dot_token_idx: TokenIdx(
                                                112,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                115,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 38,
                                            dot_token_idx: TokenIdx(
                                                116,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    117,
                                                ),
                                            },
                                        },
                                        SynExpr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                119,
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
                                                121,
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
                                                118,
                                            ),
                                            ropd: 40,
                                        },
                                        SynExpr::Field {
                                            owner: 41,
                                            dot_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    123,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 42,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                120,
                                            ),
                                            ropd: 43,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                128,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 45,
                                            dot_token_idx: TokenIdx(
                                                129,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `contour`,
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                134,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 47,
                                            dot_token_idx: TokenIdx(
                                                135,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    136,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `i1`,
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                40,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 46,
                                            dot_token_idx: TokenIdx(
                                                131,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                133,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 48,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            137,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 49,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                139,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                142,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                150,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 52,
                                            dot_token_idx: TokenIdx(
                                                147,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `cross`,
                                                token_idx: TokenIdx(
                                                    148,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                149,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 53,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                151,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                140,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 51,
                                            dot_token_idx: TokenIdx(
                                                143,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    144,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                145,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 54,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                152,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 55,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                141,
                                            ),
                                            ropd: 56,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                158,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                157,
                                            ),
                                            opd: 58,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                162,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 60,
                                            dot_token_idx: TokenIdx(
                                                163,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    164,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                166,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                168,
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
                                                167,
                                            ),
                                            ropd: 63,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 61,
                                            lbox_token_idx: TokenIdx(
                                                165,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 64,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                169,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 65,
                                            dot_token_idx: TokenIdx(
                                                170,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    171,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_interval`,
                                            token_idx: TokenIdx(
                                                173,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 67,
                                            dot_token_idx: TokenIdx(
                                                174,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    175,
                                                ),
                                            },
                                        },
                                        SynExpr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                177,
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
                                                179,
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
                                                176,
                                            ),
                                            ropd: 69,
                                        },
                                        SynExpr::Field {
                                            owner: 70,
                                            dot_token_idx: TokenIdx(
                                                180,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    181,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 71,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ropd: 72,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                186,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 74,
                                            dot_token_idx: TokenIdx(
                                                187,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `contour`,
                                                token_idx: TokenIdx(
                                                    188,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 76,
                                            dot_token_idx: TokenIdx(
                                                193,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    194,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `i2`,
                                            token_idx: TokenIdx(
                                                196,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                69,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 75,
                                            dot_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    190,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                191,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 77,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            195,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 78,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                197,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                200,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                204,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                208,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 81,
                                            dot_token_idx: TokenIdx(
                                                205,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `cross`,
                                                token_idx: TokenIdx(
                                                    206,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                207,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 82,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                209,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                198,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 80,
                                            dot_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    202,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                203,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 83,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 84,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                199,
                                            ),
                                            ropd: 85,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                212,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                214,
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
                                                213,
                                            ),
                                            ropd: 88,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                218,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                220,
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
                                                219,
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
                                                    125,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        127,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 50,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 57,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        185,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 79,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 86,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        94,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 27,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    97,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        99,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 37,
                                        },
                                        SynStmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    119,
                                                ),
                                                frame_var_expr_idx: 40,
                                                frame_var_ident: `i1`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            39,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
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
                                                            124,
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
                                                    153,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        156,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 59,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    159,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        161,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 66,
                                        },
                                        SynStmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    177,
                                                ),
                                                frame_var_expr_idx: 69,
                                                frame_var_ident: `i2`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            68,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
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
                                                            182,
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
                                                    211,
                                                ),
                                            },
                                            result: 89,
                                        },
                                        SynStmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    217,
                                                ),
                                            },
                                            result: 92,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    33,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        35,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 2,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        45,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 9,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    58,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        60,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 19,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    77,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        79,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 22,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        86,
                                                    ),
                                                },
                                                condition: Ok(
                                                    25,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                90,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        4..11,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    216,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            11..12,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `current_displacement`,
                                                    token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `previous_displacement`,
                                                    token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                92,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `previous_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        93,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                154,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `current_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `current_interval`,
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        184,
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
                                            PatternSynSymbol::Atom(
                                                0,
                                            ),
                                            PatternSynSymbol::Atom(
                                                1,
                                            ),
                                            PatternSynSymbol::Atom(
                                                2,
                                            ),
                                            PatternSynSymbol::Atom(
                                                3,
                                            ),
                                            PatternSynSymbol::Atom(
                                                4,
                                            ),
                                            PatternSynSymbol::Atom(
                                                5,
                                            ),
                                            PatternSynSymbol::Atom(
                                                6,
                                            ),
                                            PatternSynSymbol::Atom(
                                                7,
                                            ),
                                            PatternSynSymbol::Atom(
                                                8,
                                            ),
                                            PatternSynSymbol::Atom(
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
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
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
                                                    35,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
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
                                                    45,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
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
                                                    60,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
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
                                                    79,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
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
                                                    94,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
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
                                                    99,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
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
                                                    125,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            153,
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
                                                    127,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            153,
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
                                                    156,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
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
                                                    161,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
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
                                                    183,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            211,
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
                                                    185,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            211,
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