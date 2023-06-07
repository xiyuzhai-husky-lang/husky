Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ast_idx: 74,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
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
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    45,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            NoRightOperandForBinaryOperator {
                                                                punctuation: Closed(
                                                                    RemEuclid,
                                                                ),
                                                                punctuation_token_idx: TokenIdx(
                                                                    45,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            44,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            46,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                        opd: 4,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 5,
                                                        argument: 6,
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
                                                                    39,
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
                                                                ident: `LineSegmentStroke`,
                                                                token_idx: TokenIdx(
                                                                    47,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: RegularStructFieldType {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 190,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                36,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: RegularStructFieldType {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 362,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                41,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 7,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            35,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            RegularStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        37,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                            RegularStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `strokes`,
                                                    token_idx: TokenIdx(
                                                        41,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        42,
                                                    ),
                                                ),
                                                ty_expr_idx: 7,
                                                initialization: None,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    40,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    48,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            49,
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                    ast_idx: 77,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                        ),
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
                                                    Expr::Prefix {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            545,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            549,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            550,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 2,
                                                        argument: 3,
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
                                                                    546,
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
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    551,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    543,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Pure,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
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
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
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
                                                                544,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `line_segment_sketch`,
                                                                pattern_symbol_idx: 0,
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
                                                            ty: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 4,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                542,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        544,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                547,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                548,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 4,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                552,
                                            ),
                                        },
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                                ),
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
                                                            Expr::Prefix {
                                                                opr: Ref,
                                                                opr_token_idx: TokenIdx(
                                                                    545,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    549,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    2..2,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    550,
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::ExplicitApplication {
                                                                function: 2,
                                                                argument: 3,
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
                                                                            546,
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
                                                                        ident: `ConcaveComponent`,
                                                                        token_idx: TokenIdx(
                                                                            551,
                                                                        ),
                                                                    },
                                                                ),
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            543,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [
                                                                Pure,
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_arena: Arena {
                                                            data: [
                                                                PatternSymbol::Atom(
                                                                    0,
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
                                                        ],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [
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
                                                                        544,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `line_segment_sketch`,
                                                                        pattern_symbol_idx: 0,
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
                                                                    ty: 1,
                                                                },
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ExplicitParameterType,
                                                            expr_idx: 1,
                                                        },
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 4,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        557,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        558,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 0,
                                                    argument: 1,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        561,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        562,
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        566,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 4,
                                                    dot_token_idx: TokenIdx(
                                                        567,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            568,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodApplicationOrCall {
                                                    self_argument: 5,
                                                    dot_token_idx: TokenIdx(
                                                        569,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            570,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        571,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        6..6,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        572,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        577,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        582,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        587,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        584,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        586,
                                                    ),
                                                    opd: 9,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        592,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        594,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::FunctionApplicationOrCall {
                                                    function: 12,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        591,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        13..15,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            593,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        595,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 10,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        585,
                                                    ),
                                                    ropd: 11,
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        589,
                                                    ),
                                                    opd: 15,
                                                },
                                                Expr::Binary {
                                                    lopd: 16,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        588,
                                                    ),
                                                    ropd: 17,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        597,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Suffix {
                                                    opd: 19,
                                                    opr: Decr,
                                                    opr_token_idx: TokenIdx(
                                                        598,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        602,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `ccv_start`,
                                                    token_idx: TokenIdx(
                                                        606,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        608,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        604,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 22,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        607,
                                                    ),
                                                    ropd: 23,
                                                },
                                                Expr::Binary {
                                                    lopd: 24,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        605,
                                                    ),
                                                    ropd: 25,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        613,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        615,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        611,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 27,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        614,
                                                    ),
                                                    ropd: 28,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        620,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        622,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::FunctionApplicationOrCall {
                                                    function: 31,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        619,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        32..34,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            621,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        623,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 29,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        612,
                                                    ),
                                                    ropd: 30,
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        617,
                                                    ),
                                                    opd: 34,
                                                },
                                                Expr::Binary {
                                                    lopd: 35,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        616,
                                                    ),
                                                    ropd: 36,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        625,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Suffix {
                                                    opd: 38,
                                                    opr: Incr,
                                                    opr_token_idx: TokenIdx(
                                                        626,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        630,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        632,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        628,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 40,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        631,
                                                    ),
                                                    ropd: 41,
                                                },
                                                Expr::Binary {
                                                    lopd: 42,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        629,
                                                    ),
                                                    ropd: 43,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        634,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        642,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 47,
                                                    dot_token_idx: TokenIdx(
                                                        643,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            644,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        648,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        650,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        640,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::MethodApplicationOrCall {
                                                    self_argument: 48,
                                                    dot_token_idx: TokenIdx(
                                                        645,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `cyclic_slice`,
                                                        token_idx: TokenIdx(
                                                            646,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        647,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        49..51,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            649,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        651,
                                                    ),
                                                },
                                                Expr::FunctionApplicationOrCall {
                                                    function: 46,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        639,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        51..53,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            641,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        652,
                                                    ),
                                                },
                                                Expr::MethodApplicationOrCall {
                                                    self_argument: 45,
                                                    dot_token_idx: TokenIdx(
                                                        635,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            636,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        637,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        53..54,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        653,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        654,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        656,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 55,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        655,
                                                    ),
                                                    ropd: 56,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        659,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        661,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        657,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 58,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        660,
                                                    ),
                                                    ropd: 59,
                                                },
                                                Expr::Binary {
                                                    lopd: 60,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        658,
                                                    ),
                                                    ropd: 61,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        663,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        7..15,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `ConcaveComponent`,
                                                            token_idx: TokenIdx(
                                                                559,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `is_convex`,
                                                            token_idx: TokenIdx(
                                                                590,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `is_convex`,
                                                            token_idx: TokenIdx(
                                                                618,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `ConcaveComponent`,
                                                            token_idx: TokenIdx(
                                                                638,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Eval {
                                                    expr_idx: 20,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 39,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 54,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            610,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        37,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    624,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                627,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            44,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        633,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 57,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 62,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            553,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonToken(
                                                                        TokenIdx(
                                                                            556,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                2,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                560,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 3,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            563,
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
                                                                565,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 6,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            573,
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
                                                                576,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 7,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            578,
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
                                                                581,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 8,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            583,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        18,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    596,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            599,
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
                                                                601,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 21,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            603,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        26,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    609,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            3..7,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            662,
                                                        ),
                                                    },
                                                    result: 63,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        554,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `concave_components`,
                                                            token_idx: TokenIdx(
                                                                555,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `L`,
                                                            token_idx: TokenIdx(
                                                                564,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        574,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                575,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        579,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                580,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `ccv_start`,
                                                            token_idx: TokenIdx(
                                                                600,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    Pure,
                                                    Move,
                                                    Move,
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
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
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `concave_components`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `L`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `start`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `end`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `ccv_start`,
                                                        4,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                    Pure,
                                                    Mut,
                                                    Mut,
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
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            556,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    664,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `concave_components`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            565,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    664,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `L`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            576,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    664,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `start`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            581,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    664,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `end`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            601,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    664,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `ccv_start`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    LetVariables {
                                                        pattern: 0,
                                                        ty: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 2,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 3,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 6,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 7,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 8,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 20,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 21,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 39,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 54,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 57,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 62,
                                            },
                                            ExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 63,
                                            },
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 64,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    64,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::ImplBlock(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 75,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 75,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            50,
                                        ),
                                    },
                                    trai_expr: 17,
                                    for_token: TokenIdx(
                                        52,
                                    ),
                                    ty_expr: 18,
                                    items: Some(
                                        TraitForType(
                                            TraitForTypeItems {
                                                ast_idx_range: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        50,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        52,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            54,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TraitForType(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::visual::Visualize`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            ident: `Visualize`,
                                                            token_idx: TokenIdx(
                                                                51,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `ConcaveComponent`,
                                                            token_idx: TokenIdx(
                                                                53,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
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
                                            ExprRoot {
                                                kind: Trait,
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::TraitForType(
                            TraitForTypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `visualize`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TraitForTypeItem(
                            TraitForTypeItemDefn::MethodFn(
                                TraitForTypeMethodFnDefn {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            trai: TraitPath(`core::visual::Visualize`),
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TraitForTypeMethodFnDecl {
                                        path: TraitForTypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            trai: TraitPath(`core::visual::Visualize`),
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TraitForType(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `visualize`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        trai: TraitPath(`core::visual::Visualize`),
                                                        ident: `visualize`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 75,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                    },
                                                    trai_expr: 17,
                                                    for_token: TokenIdx(
                                                        52,
                                                    ),
                                                    ty_expr: 18,
                                                    items: Some(
                                                        TraitForType(
                                                            TraitForTypeItems {
                                                                ast_idx_range: ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            ast_idx: 1,
                                            ident: `visualize`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 1,
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    57,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    58,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    59,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                            },
                                        ),
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::TraitForType(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                ModuleItemPath::Trait(
                                                                                    TraitPath(`core::visual::Visualize`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::EntityPath {
                                                                        entity_path_expr: 1,
                                                                        path: Some(
                                                                            EntityPath::ModuleItem(
                                                                                ModuleItemPath::Type(
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `Visualize`,
                                                                                token_idx: TokenIdx(
                                                                                    51,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::visual::Visualize`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        path_name_token: PathNameToken::Ident(
                                                                            IdentToken {
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    53,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: Trait,
                                                                    expr_idx: 0,
                                                                },
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 1,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::TraitForType(
                                                                TraitForTypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `visualize`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::visual::Html`, `Extern`),
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
                                                                    ident: `Html`,
                                                                    token_idx: TokenIdx(
                                                                        60,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::visual::Html`, `Extern`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::TraitForType(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                        ModuleItemPath::Trait(
                                                                                            TraitPath(`core::visual::Visualize`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::EntityPath {
                                                                                entity_path_expr: 1,
                                                                                path: Some(
                                                                                    EntityPath::ModuleItem(
                                                                                        ModuleItemPath::Type(
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `Visualize`,
                                                                                        token_idx: TokenIdx(
                                                                                            51,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::visual::Visualize`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                path_name_token: PathNameToken::Ident(
                                                                                    IdentToken {
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            53,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: Trait,
                                                                            expr_idx: 0,
                                                                        },
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 1,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::TraitForType(
                                                                        TraitForTypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `visualize`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::visual::Html`, `Extern`),
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
                                                                            ident: `Html`,
                                                                            token_idx: TokenIdx(
                                                                                60,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::visual::Html`, `Extern`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TraitForType(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `visualize`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            62,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
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
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            65,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `visualize`,
                                                            token_idx: TokenIdx(
                                                                66,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            67,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            68,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 2,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        3,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::ImplBlock(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 76,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 76,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            69,
                                        ),
                                    },
                                    ty_expr: 19,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                41..53,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        69,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            71,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
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
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            ident: `ConcaveComponent`,
                                                            token_idx: TokenIdx(
                                                                70,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
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
                                            ExprRoot {
                                                kind: SelfType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MemoizedField(
                                TypeMemoizedFieldDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `norm`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    decl: TypeMemoizedFieldDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `norm`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `norm`,
                                                        item_kind: MemoizedField,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 41,
                                            ident: `norm`,
                                            associated_item_kind: TypeItem(
                                                MemoizedField,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 41,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    74,
                                                ),
                                            ),
                                        ),
                                        memo_ty: Some(
                                            FormTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eq_token: EqToken(
                                            TokenIdx(
                                                76,
                                            ),
                                        ),
                                        expr: None,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `norm`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                                    ident: `f32`,
                                                                    token_idx: TokenIdx(
                                                                        75,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: VarType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `norm`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                                            ident: `f32`,
                                                                            token_idx: TokenIdx(
                                                                                75,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: VarType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            77,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            78,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `hausdorff_norm`,
                                                            token_idx: TokenIdx(
                                                                79,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 1,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 2,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        2,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `rel_norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MemoizedField(
                                TypeMemoizedFieldDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `rel_norm`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    decl: TypeMemoizedFieldDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `rel_norm`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `rel_norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `rel_norm`,
                                                        item_kind: MemoizedField,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 42,
                                            ident: `rel_norm`,
                                            associated_item_kind: TypeItem(
                                                MemoizedField,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 42,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    82,
                                                ),
                                            ),
                                        ),
                                        memo_ty: Some(
                                            FormTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eq_token: EqToken(
                                            TokenIdx(
                                                84,
                                            ),
                                        ),
                                        expr: None,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `rel_norm`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                                    ident: `f32`,
                                                                    token_idx: TokenIdx(
                                                                        83,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: VarType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `rel_norm`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                                            ident: `f32`,
                                                                            token_idx: TokenIdx(
                                                                                83,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: VarType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `rel_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            85,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            89,
                                                        ),
                                                    ),
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                91,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            93,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            86,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                87,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            94,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                95,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Closed(
                                                            Div,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            88,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 5,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 5,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 6,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        6,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `hausdorff_norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MemoizedField(
                                TypeMemoizedFieldDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `hausdorff_norm`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    decl: TypeMemoizedFieldDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `hausdorff_norm`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `hausdorff_norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `hausdorff_norm`,
                                                        item_kind: MemoizedField,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 43,
                                            ident: `hausdorff_norm`,
                                            associated_item_kind: TypeItem(
                                                MemoizedField,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 43,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    100,
                                                ),
                                            ),
                                        ),
                                        memo_ty: Some(
                                            FormTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eq_token: EqToken(
                                            TokenIdx(
                                                102,
                                            ),
                                        ),
                                        expr: None,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `hausdorff_norm`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                                    ident: `f32`,
                                                                    token_idx: TokenIdx(
                                                                        101,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: VarType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `hausdorff_norm`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                                            ident: `f32`,
                                                                            token_idx: TokenIdx(
                                                                                101,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: VarType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `hausdorff_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            107,
                                                        ),
                                                        Literal::Float(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            111,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                113,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `first`,
                                                            token_idx: TokenIdx(
                                                                115,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            116,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 3,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            118,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 4,
                                                        dot_token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                120,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            124,
                                                        ),
                                                    ),
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            125,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `line_segment`,
                                                            token_idx: TokenIdx(
                                                                126,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            127,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            7..7,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            128,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `curve_ls`,
                                                        token_idx: TokenIdx(
                                                            132,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 8,
                                                        dot_token_idx: TokenIdx(
                                                            133,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                134,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            135,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..9,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            136,
                                                        ),
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 9,
                                                        dot_token_idx: TokenIdx(
                                                            137,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                138,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            139,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            10..10,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            140,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            142,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 11,
                                                        dot_token_idx: TokenIdx(
                                                            143,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                144,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 12,
                                                        dot_token_idx: TokenIdx(
                                                            145,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                146,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            14,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            150,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 15,
                                                        dot_token_idx: TokenIdx(
                                                            151,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                152,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 13,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            147,
                                                        ),
                                                        ropd: 14,
                                                    },
                                                    Expr::Field {
                                                        owner: 16,
                                                        dot_token_idx: TokenIdx(
                                                            153,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                154,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 17,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            149,
                                                        ),
                                                        ropd: 18,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            159,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 20,
                                                        dot_token_idx: TokenIdx(
                                                            160,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                161,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            163,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            14,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 21,
                                                        lbox_token_idx: TokenIdx(
                                                            162,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            22..23,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 23,
                                                        dot_token_idx: TokenIdx(
                                                            165,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                166,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `curve_ls`,
                                                        token_idx: TokenIdx(
                                                            170,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            174,
                                                        ),
                                                        current_symbol_idx: 5,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 25,
                                                        dot_token_idx: TokenIdx(
                                                            171,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `dist_to_point`,
                                                            token_idx: TokenIdx(
                                                                172,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            173,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            26..27,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            175,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point_dist`,
                                                        token_idx: TokenIdx(
                                                            177,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `hausdorff_norm`,
                                                        token_idx: TokenIdx(
                                                            179,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 28,
                                                        opr: Comparison(
                                                            Greater,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            178,
                                                        ),
                                                        ropd: 29,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `hausdorff_norm`,
                                                        token_idx: TokenIdx(
                                                            181,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point_dist`,
                                                        token_idx: TokenIdx(
                                                            183,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 31,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            182,
                                                        ),
                                                        ropd: 32,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `hausdorff_norm`,
                                                        token_idx: TokenIdx(
                                                            185,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            4..10,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 33,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                156,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern_expr_idx: 4,
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
                                                                    158,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 24,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                167,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern_expr_idx: 5,
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
                                                            EqToken(
                                                                TokenIdx(
                                                                    169,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 27,
                                                    },
                                                    Stmt::IfElse {
                                                        if_branch: IfBranch {
                                                            if_token: IfToken {
                                                                token_idx: TokenIdx(
                                                                    176,
                                                                ),
                                                            },
                                                            condition: Ok(
                                                                30,
                                                            ),
                                                            eol_colon: Ok(
                                                                Colon(
                                                                    EolColonToken {
                                                                        token_idx: TokenIdx(
                                                                            180,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            block: Ok(
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                        },
                                                        elif_branches: [],
                                                        else_branch: None,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                103,
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
                                                                    106,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 0,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                108,
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
                                                                    110,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 5,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                121,
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
                                                                    123,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 7,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                129,
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
                                                                    131,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 10,
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                141,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                148,
                                                            ),
                                                            frame_var_expr_idx: 14,
                                                            frame_var_ident: `i`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        13,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        18,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 4,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        155,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                1..4,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                184,
                                                            ),
                                                        },
                                                        result: 34,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            104,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `hausdorff_norm`,
                                                                token_idx: TokenIdx(
                                                                    105,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `curve_start`,
                                                                token_idx: TokenIdx(
                                                                    109,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `curve_ls`,
                                                                token_idx: TokenIdx(
                                                                    122,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `dp_norm`,
                                                                token_idx: TokenIdx(
                                                                    130,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `point`,
                                                                token_idx: TokenIdx(
                                                                    157,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `point_dist`,
                                                                token_idx: TokenIdx(
                                                                    168,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Move,
                                                        Pure,
                                                        Pure,
                                                        Pure,
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
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `hausdorff_norm`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `curve_start`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `curve_ls`,
                                                            2,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `dp_norm`,
                                                            3,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `point`,
                                                            4,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `point_dist`,
                                                            5,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
                                                        Pure,
                                                        Pure,
                                                        Pure,
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
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                106,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        186,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `hausdorff_norm`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                110,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        186,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `curve_start`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                123,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        186,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `curve_ls`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                131,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        186,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `dp_norm`,
                                                                pattern_symbol_idx: 3,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                156,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        184,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 14,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                158,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        184,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `point`,
                                                                pattern_symbol_idx: 4,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                169,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        184,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `point_dist`,
                                                                pattern_symbol_idx: 5,
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
                                                            4..5,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 0,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 5,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 7,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 10,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 24,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 27,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 33,
                                                },
                                                ExprRoot {
                                                    kind: ReturnExpr,
                                                    expr_idx: 34,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 35,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        35,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `angle_change`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MemoizedField(
                                TypeMemoizedFieldDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `angle_change`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    decl: TypeMemoizedFieldDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `angle_change`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `angle_change`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `angle_change`,
                                                        item_kind: MemoizedField,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 44,
                                            ident: `angle_change`,
                                            associated_item_kind: TypeItem(
                                                MemoizedField,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 44,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    188,
                                                ),
                                            ),
                                        ),
                                        memo_ty: Some(
                                            FormTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eq_token: EqToken(
                                            TokenIdx(
                                                190,
                                            ),
                                        ),
                                        expr: None,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `angle_change`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                                    ident: `f32`,
                                                                    token_idx: TokenIdx(
                                                                        189,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: VarType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `angle_change`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                                            ident: `f32`,
                                                                            token_idx: TokenIdx(
                                                                                189,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: VarType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `angle_change`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            195,
                                                        ),
                                                        Literal::Float(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            200,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            201,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                202,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            205,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                206,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 4,
                                                        dot_token_idx: TokenIdx(
                                                            207,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                208,
                                                            ),
                                                        },
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 2,
                                                        lbox_token_idx: TokenIdx(
                                                            203,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            5..6,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            209,
                                                        ),
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            210,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                211,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            212,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            7..7,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            213,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 8,
                                                        dot_token_idx: TokenIdx(
                                                            216,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                217,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 9,
                                                        dot_token_idx: TokenIdx(
                                                            218,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                219,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            221,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            11,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            223,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 12,
                                                        dot_token_idx: TokenIdx(
                                                            224,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                225,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 10,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            220,
                                                        ),
                                                        ropd: 11,
                                                    },
                                                    Expr::Field {
                                                        owner: 13,
                                                        dot_token_idx: TokenIdx(
                                                            226,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                227,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 14,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            222,
                                                        ),
                                                        ropd: 15,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            232,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 17,
                                                        dot_token_idx: TokenIdx(
                                                            233,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                234,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            236,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            11,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 18,
                                                        lbox_token_idx: TokenIdx(
                                                            235,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            19..20,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            237,
                                                        ),
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 20,
                                                        dot_token_idx: TokenIdx(
                                                            238,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                239,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            240,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            21..21,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            241,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp0`,
                                                        token_idx: TokenIdx(
                                                            244,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp`,
                                                        token_idx: TokenIdx(
                                                            248,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            250,
                                                        ),
                                                        Literal::Bool(
                                                            True,
                                                        ),
                                                    ),
                                                    Expr::CurrentSymbol {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            242,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 22,
                                                        dot_token_idx: TokenIdx(
                                                            245,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `angle_to`,
                                                            token_idx: TokenIdx(
                                                                246,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            247,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            23..25,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                249,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            251,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 25,
                                                        opr: AssignClosed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            243,
                                                        ),
                                                        ropd: 26,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp0`,
                                                        token_idx: TokenIdx(
                                                            252,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp`,
                                                        token_idx: TokenIdx(
                                                            254,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 28,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            253,
                                                        ),
                                                        ropd: 29,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            256,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            3..7,
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
                                                                229,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern_expr_idx: 2,
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
                                                                    231,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 21,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 27,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 30,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                191,
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
                                                                    194,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 0,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                196,
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
                                                                    199,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 7,
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                214,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                221,
                                                            ),
                                                            frame_var_expr_idx: 11,
                                                            frame_var_ident: `i`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        10,
                                                                    ),
                                                                    kind: LowerOpen,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        15,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 2,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        228,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                0..3,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                255,
                                                            ),
                                                        },
                                                        result: 31,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            192,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `angle_change`,
                                                                token_idx: TokenIdx(
                                                                    193,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            197,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `dp0`,
                                                                token_idx: TokenIdx(
                                                                    198,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `dp`,
                                                                token_idx: TokenIdx(
                                                                    230,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Move,
                                                        Move,
                                                        Pure,
                                                    ],
                                                },
                                                pattern_infos: [
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
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `angle_change`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `dp0`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `dp`,
                                                            2,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
                                                        Mut,
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
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                194,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        257,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `angle_change`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                199,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        257,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `dp0`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                229,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        255,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 11,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                231,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        255,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `dp`,
                                                                pattern_symbol_idx: 2,
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
                                                            2..3,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 0,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 7,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 21,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 27,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 30,
                                                },
                                                ExprRoot {
                                                    kind: ReturnExpr,
                                                    expr_idx: 31,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 32,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        32,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `bounding_box`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MemoizedField(
                                TypeMemoizedFieldDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    decl: TypeMemoizedFieldDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `bounding_box`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `bounding_box`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `bounding_box`,
                                                        item_kind: MemoizedField,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 45,
                                            ident: `bounding_box`,
                                            associated_item_kind: TypeItem(
                                                MemoizedField,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 45,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    259,
                                                ),
                                            ),
                                        ),
                                        memo_ty: Some(
                                            FormTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eq_token: EqToken(
                                            TokenIdx(
                                                261,
                                            ),
                                        ),
                                        expr: None,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `bounding_box`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                    ident: `BoundingBox`,
                                                                    token_idx: TokenIdx(
                                                                        260,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: VarType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `bounding_box`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                            ident: `BoundingBox`,
                                                                            token_idx: TokenIdx(
                                                                                260,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: VarType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `bounding_box`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            265,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            266,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                267,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            268,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `first`,
                                                            token_idx: TokenIdx(
                                                                269,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            270,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            271,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 2,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            272,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            273,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                274,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            279,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 5,
                                                        dot_token_idx: TokenIdx(
                                                            280,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                281,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            286,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            287,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                288,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            293,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 9,
                                                        dot_token_idx: TokenIdx(
                                                            294,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                295,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            300,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 11,
                                                        dot_token_idx: TokenIdx(
                                                            301,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                302,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            304,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 13,
                                                        dot_token_idx: TokenIdx(
                                                            305,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                306,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 14,
                                                        dot_token_idx: TokenIdx(
                                                            307,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                308,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            310,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 5,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            16,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            312,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 17,
                                                        dot_token_idx: TokenIdx(
                                                            313,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                314,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 15,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            309,
                                                        ),
                                                        ropd: 16,
                                                    },
                                                    Expr::Field {
                                                        owner: 18,
                                                        dot_token_idx: TokenIdx(
                                                            315,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                316,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 19,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            311,
                                                        ),
                                                        ropd: 20,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            321,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 22,
                                                        dot_token_idx: TokenIdx(
                                                            322,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                323,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            325,
                                                        ),
                                                        current_symbol_idx: 5,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            16,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 23,
                                                        lbox_token_idx: TokenIdx(
                                                            324,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            24..25,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            326,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 25,
                                                        dot_token_idx: TokenIdx(
                                                            327,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                328,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmin`,
                                                        token_idx: TokenIdx(
                                                            331,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            335,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 28,
                                                        dot_token_idx: TokenIdx(
                                                            336,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                337,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmin`,
                                                        token_idx: TokenIdx(
                                                            329,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 27,
                                                        dot_token_idx: TokenIdx(
                                                            332,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `min`,
                                                            token_idx: TokenIdx(
                                                                333,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            334,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            29..30,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            338,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 30,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            330,
                                                        ),
                                                        ropd: 31,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmax`,
                                                        token_idx: TokenIdx(
                                                            341,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            345,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 34,
                                                        dot_token_idx: TokenIdx(
                                                            346,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                347,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmax`,
                                                        token_idx: TokenIdx(
                                                            339,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 33,
                                                        dot_token_idx: TokenIdx(
                                                            342,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                343,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            344,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            35..36,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            348,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 36,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            340,
                                                        ),
                                                        ropd: 37,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            351,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            355,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 40,
                                                        dot_token_idx: TokenIdx(
                                                            356,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                357,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            349,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 39,
                                                        dot_token_idx: TokenIdx(
                                                            352,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `min`,
                                                            token_idx: TokenIdx(
                                                                353,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            354,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            41..42,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            358,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 42,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            350,
                                                        ),
                                                        ropd: 43,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            361,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            365,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 46,
                                                        dot_token_idx: TokenIdx(
                                                            366,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                367,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            359,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 45,
                                                        dot_token_idx: TokenIdx(
                                                            362,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                363,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            364,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            47..48,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            368,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 48,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            360,
                                                        ),
                                                        ropd: 49,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmin`,
                                                        token_idx: TokenIdx(
                                                            374,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmax`,
                                                        token_idx: TokenIdx(
                                                            376,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            381,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            383,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::FunctionApplicationOrCall {
                                                        function: 52,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            373,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            53..55,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                375,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            377,
                                                        ),
                                                    },
                                                    Expr::FunctionApplicationOrCall {
                                                        function: 55,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            380,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            56..58,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                382,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            384,
                                                        ),
                                                    },
                                                    Expr::FunctionApplicationOrCall {
                                                        function: 51,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            371,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            58..60,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                378,
                                                            ),
                                                            TokenIdx(
                                                                385,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            386,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            5..12,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `BoundingBox`,
                                                                token_idx: TokenIdx(
                                                                    370,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ClosedRange`,
                                                                token_idx: TokenIdx(
                                                                    372,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ClosedRange`,
                                                                token_idx: TokenIdx(
                                                                    379,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                318,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern_expr_idx: 5,
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
                                                            EqToken(
                                                                TokenIdx(
                                                                    320,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 26,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 32,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 38,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 44,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 50,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                262,
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
                                                                    264,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 4,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                275,
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
                                                                    278,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 6,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                282,
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
                                                                    285,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 8,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                289,
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
                                                                    292,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 10,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                296,
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
                                                                    299,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 12,
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                303,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                310,
                                                            ),
                                                            frame_var_expr_idx: 16,
                                                            frame_var_ident: `i`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        15,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        20,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 5,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        317,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                0..5,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                369,
                                                            ),
                                                        },
                                                        result: 60,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `start_point`,
                                                                token_idx: TokenIdx(
                                                                    263,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            276,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `xmin`,
                                                                token_idx: TokenIdx(
                                                                    277,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            283,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `xmax`,
                                                                token_idx: TokenIdx(
                                                                    284,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            290,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `ymin`,
                                                                token_idx: TokenIdx(
                                                                    291,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            297,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `ymax`,
                                                                token_idx: TokenIdx(
                                                                    298,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `point`,
                                                                token_idx: TokenIdx(
                                                                    319,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Pure,
                                                        Move,
                                                        Move,
                                                        Move,
                                                        Move,
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
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `start_point`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `xmin`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `xmax`,
                                                            2,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `ymin`,
                                                            3,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `ymax`,
                                                            4,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `point`,
                                                            5,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Pure,
                                                        Mut,
                                                        Mut,
                                                        Mut,
                                                        Mut,
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
                                                                264,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        387,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `start_point`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                278,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        387,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `xmin`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                285,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        387,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `xmax`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                292,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        387,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `ymin`,
                                                                pattern_symbol_idx: 3,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                299,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        387,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `ymax`,
                                                                pattern_symbol_idx: 4,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                318,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        369,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 16,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                320,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        369,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `point`,
                                                                pattern_symbol_idx: 5,
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
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 6,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 8,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 10,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 12,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 26,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 32,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 38,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 44,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 50,
                                                },
                                                ExprRoot {
                                                    kind: ReturnExpr,
                                                    expr_idx: 60,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 61,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        61,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `relative_bounding_box`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MemoizedField(
                                TypeMemoizedFieldDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    decl: TypeMemoizedFieldDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `relative_bounding_box`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `relative_bounding_box`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `relative_bounding_box`,
                                                        item_kind: MemoizedField,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 46,
                                            ident: `relative_bounding_box`,
                                            associated_item_kind: TypeItem(
                                                MemoizedField,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 46,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    389,
                                                ),
                                            ),
                                        ),
                                        memo_ty: Some(
                                            FormTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eq_token: EqToken(
                                            TokenIdx(
                                                391,
                                            ),
                                        ),
                                        expr: None,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `relative_bounding_box`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                    ident: `RelativeBoundingBox`,
                                                                    token_idx: TokenIdx(
                                                                        390,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: VarType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `relative_bounding_box`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                            ident: `RelativeBoundingBox`,
                                                                            token_idx: TokenIdx(
                                                                                390,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: VarType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `relative_bounding_box`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            392,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            393,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `line_segment_sketch`,
                                                            token_idx: TokenIdx(
                                                                394,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            395,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `bounding_box`,
                                                            token_idx: TokenIdx(
                                                                396,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            400,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            401,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `bounding_box`,
                                                            token_idx: TokenIdx(
                                                                402,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            397,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `relative_bounding_box`,
                                                            token_idx: TokenIdx(
                                                                398,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            399,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            4..5,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            403,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 5,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 5,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 6,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        6,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `line_segment`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `line_segment`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `line_segment`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `line_segment`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `line_segment`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `line_segment`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 47,
                                            ident: `line_segment`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 47,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `line_segment`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                    ident: `LineSegment`,
                                                                    token_idx: TokenIdx(
                                                                        409,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    406,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    407,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    408,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    410,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `line_segment`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                            ident: `LineSegment`,
                                                                            token_idx: TokenIdx(
                                                                                409,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `line_segment`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            413,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            414,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                415,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            416,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `first`,
                                                            token_idx: TokenIdx(
                                                                417,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            418,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            419,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 3,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            420,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 4,
                                                        dot_token_idx: TokenIdx(
                                                            421,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                422,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            428,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 6,
                                                        dot_token_idx: TokenIdx(
                                                            429,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                430,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 7,
                                                        dot_token_idx: TokenIdx(
                                                            431,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `last`,
                                                            token_idx: TokenIdx(
                                                                432,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            433,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            8..8,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            434,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 8,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            435,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 9,
                                                        dot_token_idx: TokenIdx(
                                                            436,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                437,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 5,
                                                        dot_token_idx: TokenIdx(
                                                            423,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                424,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            425,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            6..6,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            426,
                                                        ),
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 10,
                                                        dot_token_idx: TokenIdx(
                                                            438,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                439,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            440,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            11..11,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            441,
                                                        ),
                                                    },
                                                    Expr::FunctionApplicationOrCall {
                                                        function: 0,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            412,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            11..13,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                427,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            442,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `LineSegment`,
                                                                token_idx: TokenIdx(
                                                                    411,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 13,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 13,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 14,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        14,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `start`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `start`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `start`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `start`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `start`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `start`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 48,
                                            ident: `start`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 48,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `start`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                    ident: `Point2d`,
                                                                    token_idx: TokenIdx(
                                                                        448,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    445,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    446,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    447,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    449,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `start`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                            ident: `Point2d`,
                                                                            token_idx: TokenIdx(
                                                                                448,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `start`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            450,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            451,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                452,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            453,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `first`,
                                                            token_idx: TokenIdx(
                                                                454,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            455,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            456,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 2,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            457,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            458,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                459,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 4,
                                                        dot_token_idx: TokenIdx(
                                                            460,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                461,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            462,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            5..5,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            463,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 5,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 5,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 6,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        6,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `end`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `end`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `end`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `end`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `end`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `end`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 49,
                                            ident: `end`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 49,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `end`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                    ident: `Point2d`,
                                                                    token_idx: TokenIdx(
                                                                        469,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    466,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    467,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    468,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    470,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `end`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                            ident: `Point2d`,
                                                                            token_idx: TokenIdx(
                                                                                469,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `end`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            471,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            472,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                473,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            474,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `last`,
                                                            token_idx: TokenIdx(
                                                                475,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            476,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            477,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 2,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            478,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            479,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                480,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 4,
                                                        dot_token_idx: TokenIdx(
                                                            481,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                482,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            483,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            5..5,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            484,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 5,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 5,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 6,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        6,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `displacement`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `displacement`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `displacement`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `displacement`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `displacement`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `displacement`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 50,
                                            ident: `displacement`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 50,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `displacement`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                    ident: `Vector2d`,
                                                                    token_idx: TokenIdx(
                                                                        490,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    487,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    488,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    489,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    491,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `displacement`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            ident: `Vector2d`,
                                                                            token_idx: TokenIdx(
                                                                                490,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `displacement`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            492,
                                                        ),
                                                    ),
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 0,
                                                        dot_token_idx: TokenIdx(
                                                            493,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `line_segment`,
                                                            token_idx: TokenIdx(
                                                                494,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            495,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            496,
                                                        ),
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            497,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                498,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            499,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            500,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 2,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        3,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `start_tangent`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `start_tangent`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `start_tangent`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `start_tangent`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `start_tangent`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `start_tangent`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 51,
                                            ident: `start_tangent`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 51,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `start_tangent`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                    ident: `Vector2d`,
                                                                    token_idx: TokenIdx(
                                                                        506,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    503,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    504,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    505,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    507,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `start_tangent`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            ident: `Vector2d`,
                                                                            token_idx: TokenIdx(
                                                                                506,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `start_tangent`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            508,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            509,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                510,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            511,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `first`,
                                                            token_idx: TokenIdx(
                                                                512,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            513,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            514,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 2,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            515,
                                                        ),
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 3,
                                                        dot_token_idx: TokenIdx(
                                                            516,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                517,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            518,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            519,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 4,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 5,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        5,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `end_tangent`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `end_tangent`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `end_tangent`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `end_tangent`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `end_tangent`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `end_tangent`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 76,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                    ty_expr: 19,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                41..53,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 52,
                                            ident: `end_tangent`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 52,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::ImplBlock(
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
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
                                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                ident: `ConcaveComponent`,
                                                                                token_idx: TokenIdx(
                                                                                    70,
                                                                                ),
                                                                            },
                                                                        ),
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    data: [],
                                                                },
                                                                pattern_expr_contracts: ArenaMap {
                                                                    data: [],
                                                                },
                                                                pattern_infos: [],
                                                                pattern_symbol_arena: Arena {
                                                                    data: [],
                                                                },
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_modifiers: ArenaMap {
                                                                    data: [],
                                                                },
                                                            },
                                                            symbol_region: SymbolRegion {
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
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `end_tangent`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                    ident: `Vector2d`,
                                                                    token_idx: TokenIdx(
                                                                        525,
                                                                    ),
                                                                },
                                                            ),
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
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
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    522,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    523,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    524,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    526,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                    disambiguator: 0,
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
                                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                        ident: `ConcaveComponent`,
                                                                                        token_idx: TokenIdx(
                                                                                            70,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            data: [],
                                                                        },
                                                                        pattern_expr_contracts: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                        pattern_infos: [],
                                                                        pattern_symbol_arena: Arena {
                                                                            data: [],
                                                                        },
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_modifiers: ArenaMap {
                                                                            data: [],
                                                                        },
                                                                    },
                                                                    symbol_region: SymbolRegion {
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
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `end_tangent`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            ident: `Vector2d`,
                                                                            token_idx: TokenIdx(
                                                                                525,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
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
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `end_tangent`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            527,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            528,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                529,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            530,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `last`,
                                                            token_idx: TokenIdx(
                                                                531,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            532,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            533,
                                                        ),
                                                    },
                                                    Expr::Suffix {
                                                        opd: 2,
                                                        opr: UnwrapOrComposeWithNot,
                                                        opr_token_idx: TokenIdx(
                                                            534,
                                                        ),
                                                    },
                                                    Expr::MethodApplicationOrCall {
                                                        self_argument: 3,
                                                        dot_token_idx: TokenIdx(
                                                            535,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                536,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            537,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            538,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 4,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
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
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 5,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        5,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)