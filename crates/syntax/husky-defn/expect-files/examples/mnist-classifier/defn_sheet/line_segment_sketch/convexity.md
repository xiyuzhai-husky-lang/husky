Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                            implicit_parameters: [],
                            regular_parameters: [
                                RegularParameterDeclPattern {
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
                                RegularParameterDeclPattern {
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
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 2,
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
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
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::basic::bool`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `LineSegmentSketch`,
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `bool`,
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
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
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Ident {
                                                    modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `line_segment_sketch`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                                PatternExpr::Ident {
                                                    modifier_keyword_group: None,
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
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternSymbol::Atom(
                                                    0,
                                                ),
                                                PatternSymbol::Atom(
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
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        27,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                ExplicitParameter {
                                                    pattern_expr: 0,
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ExplicitParameter {
                                                    pattern_expr: 1,
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 0,
                                        },
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
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
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
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
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::basic::bool`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `LineSegmentSketch`,
                                                                token_idx: TokenIdx(
                                                                    24,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `i32`,
                                                                token_idx: TokenIdx(
                                                                    28,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `bool`,
                                                                token_idx: TokenIdx(
                                                                    31,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
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
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `line_segment_sketch`,
                                                                token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
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
                                                        Pure,
                                                        Pure,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
                                                        PatternSymbol::Atom(
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
                                                        Pure,
                                                        Pure,
                                                    ],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                23,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `line_segment_sketch`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                27,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 0,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 1,
                                                            ty: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 0,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
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
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                36,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                41,
                                            ),
                                            items: ArenaIdxRange(
                                                2..2,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                42,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                46,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                50,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                52,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                51,
                                            ),
                                            ropd: 6,
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 4,
                                            lbox_token_idx: TokenIdx(
                                                49,
                                            ),
                                            items: ArenaIdxRange(
                                                7..8,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                53,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                56,
                                            ),
                                            items: ArenaIdxRange(
                                                9..9,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                57,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                61,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                68,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                65,
                                            ),
                                            item: 14,
                                            rpar_token_idx: TokenIdx(
                                                69,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                71,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 15,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                70,
                                            ),
                                            ropd: 16,
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 11,
                                            lbox_token_idx: TokenIdx(
                                                64,
                                            ),
                                            items: ArenaIdxRange(
                                                17..18,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                72,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                75,
                                            ),
                                            items: ArenaIdxRange(
                                                19..19,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                76,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_displacement`,
                                            token_idx: TokenIdx(
                                                80,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                            items: ArenaIdxRange(
                                                21..22,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                85,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                87,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                89,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 23,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                88,
                                            ),
                                            ropd: 24,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                96,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                95,
                                            ),
                                            opd: 26,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                100,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                105,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                107,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 30,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                106,
                                            ),
                                            ropd: 31,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                104,
                                            ),
                                            item: 32,
                                            rpar_token_idx: TokenIdx(
                                                108,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                110,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 33,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                109,
                                            ),
                                            ropd: 34,
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 29,
                                            lbox_token_idx: TokenIdx(
                                                103,
                                            ),
                                            items: ArenaIdxRange(
                                                35..36,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                111,
                                            ),
                                        },
                                        Expr::Field {
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
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                115,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident: `i1`,
                                            frame_var_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                40,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                121,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 39,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                118,
                                            ),
                                            ropd: 40,
                                        },
                                        Expr::Field {
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
                                        Expr::Binary {
                                            lopd: 42,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                120,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                128,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                134,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::CurrentSymbol {
                                            ident: `i1`,
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                40,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                133,
                                            ),
                                            items: ArenaIdxRange(
                                                48..50,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    137,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                139,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                142,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                150,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                149,
                                            ),
                                            items: ArenaIdxRange(
                                                53..54,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                151,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                140,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                145,
                                            ),
                                            items: ArenaIdxRange(
                                                54..55,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                152,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 55,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                141,
                                            ),
                                            ropd: 56,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                158,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                157,
                                            ),
                                            opd: 58,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                162,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                166,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `index`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                168,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 62,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                167,
                                            ),
                                            ropd: 63,
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 61,
                                            lbox_token_idx: TokenIdx(
                                                165,
                                            ),
                                            items: ArenaIdxRange(
                                                64..65,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                169,
                                            ),
                                        },
                                        Expr::Field {
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
                                        Expr::CurrentSymbol {
                                            ident: `current_interval`,
                                            token_idx: TokenIdx(
                                                173,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                177,
                                            ),
                                            ident: `i2`,
                                            frame_var_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                69,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_interval`,
                                            token_idx: TokenIdx(
                                                179,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 68,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                176,
                                            ),
                                            ropd: 69,
                                        },
                                        Expr::Field {
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
                                        Expr::Binary {
                                            lopd: 71,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ropd: 72,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                186,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `line_segment_sketch`,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Field {
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
                                        Expr::CurrentSymbol {
                                            ident: `i2`,
                                            token_idx: TokenIdx(
                                                196,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                69,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                191,
                                            ),
                                            items: ArenaIdxRange(
                                                77..79,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    195,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                197,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                200,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                204,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                208,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                207,
                                            ),
                                            items: ArenaIdxRange(
                                                82..83,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                209,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                198,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
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
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                203,
                                            ),
                                            items: ArenaIdxRange(
                                                83..84,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 84,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                199,
                                            ),
                                            ropd: 85,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                212,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                214,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 87,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                213,
                                            ),
                                            ropd: 88,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                218,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                220,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 90,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                219,
                                            ),
                                            ropd: 91,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                12..17,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    125,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::Eval {
                                            expr_idx: 57,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::Eval {
                                            expr_idx: 86,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    97,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::ForBetween {
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
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    153,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    159,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::ForBetween {
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
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    211,
                                                ),
                                            },
                                            result: 89,
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    217,
                                                ),
                                            },
                                            result: 92,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    33,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    58,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    77,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                        Stmt::IfElse {
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
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `current_displacement`,
                                                    token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `previous_displacement`,
                                                    token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: Some(
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
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: Some(
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
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `current_interval`,
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
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
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Move,
                                            Pure,
                                            Pure,
                                            Move,
                                            Pure,
                                            Pure,
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
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                            PatternSymbol::Atom(
                                                1,
                                            ),
                                            PatternSymbol::Atom(
                                                2,
                                            ),
                                            PatternSymbol::Atom(
                                                3,
                                            ),
                                            PatternSymbol::Atom(
                                                4,
                                            ),
                                            PatternSymbol::Atom(
                                                5,
                                            ),
                                            PatternSymbol::Atom(
                                                6,
                                            ),
                                            PatternSymbol::Atom(
                                                7,
                                            ),
                                            PatternSymbol::Atom(
                                                8,
                                            ),
                                            PatternSymbol::Atom(
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
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Mut,
                                            Pure,
                                            Pure,
                                            Mut,
                                            Pure,
                                            Pure,
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `line_segment_sketch`,
                                                },
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `index`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `L`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `current_displacement`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `previous_displacement`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `previous_raw_cross`,
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `previous_interval`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i1`,
                                                    expr_idx: 40,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `displacement`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `current_raw_cross`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `current_interval`,
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i2`,
                                                    expr_idx: 69,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
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
                                                variant: CurrentSymbolVariant::LetVariable {
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
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 19,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 22,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 27,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 37,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 50,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 57,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 59,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 66,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 79,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 86,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 89,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 92,
                                    },
                                    ExprRoot {
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