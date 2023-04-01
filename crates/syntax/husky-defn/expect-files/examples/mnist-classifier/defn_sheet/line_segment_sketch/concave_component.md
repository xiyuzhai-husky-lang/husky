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
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    44,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoRightOperandForBinaryOperator {
                                                                punctuation: Closed(
                                                                    RemEuclid,
                                                                ),
                                                                punctuation_token_idx: TokenIdx(
                                                                    44,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            45,
                                                        ),
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
                                                        function: 4,
                                                        argument: 5,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            39,
                                                        ),
                                                        ident: `LineSegmentSketch`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            46,
                                                        ),
                                                        ident: `LineSegmentStroke`,
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    kind: FieldType,
                                                    expr: 1,
                                                },
                                                ExprRoot {
                                                    kind: FieldType,
                                                    expr: 6,
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
                                            FieldDeclPattern {
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
                                                ty: 1,
                                            },
                                            FieldDeclPattern {
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
                                                ty: 6,
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
                                                    47,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            48,
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
                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                    ast_idx: 76,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
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
                                                            526,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            530,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            531,
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
                                                        token_idx: TokenIdx(
                                                            527,
                                                        ),
                                                        ident: `LineSegmentSketch`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            532,
                                                        ),
                                                        ident: `ConcaveComponent`,
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
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `line_segment_sketch`,
                                                                token_idx: TokenIdx(
                                                                    524,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `line_segment_sketch`,
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            access_start: TokenIdx(
                                                                525,
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
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                523,
                                            ),
                                        ),
                                        self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        525,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                528,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                529,
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
                                                533,
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
                                                                    FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
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
                                                                    526,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    530,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    2..2,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    531,
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
                                                                token_idx: TokenIdx(
                                                                    527,
                                                                ),
                                                                ident: `LineSegmentSketch`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    532,
                                                                ),
                                                                ident: `ConcaveComponent`,
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
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `line_segment_sketch`,
                                                                        token_idx: TokenIdx(
                                                                            524,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `line_segment_sketch`,
                                                                    0,
                                                                ),
                                                            ],
                                                        ],
                                                        pattern_symbol_arena: Arena {
                                                            data: [
                                                                PatternSymbol::Atom(
                                                                    0,
                                                                ),
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
                                                                    access_start: TokenIdx(
                                                                        525,
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
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 1,
                                                            },
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr: 4,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        538,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        539,
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
                                                        542,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        543,
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        547,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 4,
                                                    dot_token_idx: TokenIdx(
                                                        548,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            549,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 5,
                                                    dot_token_idx: TokenIdx(
                                                        550,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            551,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        552,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        6..6,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        553,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        558,
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        563,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        568,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        565,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        567,
                                                    ),
                                                    opd: 9,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        573,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        575,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 12,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        572,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        13..15,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            574,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        576,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 10,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        566,
                                                    ),
                                                    ropd: 11,
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        570,
                                                    ),
                                                    opd: 15,
                                                },
                                                Expr::Binary {
                                                    lopd: 16,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        569,
                                                    ),
                                                    ropd: 17,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        578,
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
                                                        579,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        583,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `ccv_start`,
                                                    token_idx: TokenIdx(
                                                        587,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        589,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        585,
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
                                                        588,
                                                    ),
                                                    ropd: 23,
                                                },
                                                Expr::Binary {
                                                    lopd: 24,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        586,
                                                    ),
                                                    ropd: 25,
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
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        596,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        592,
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
                                                        595,
                                                    ),
                                                    ropd: 28,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        601,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        603,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 31,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        600,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        32..34,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            602,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        604,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 29,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        593,
                                                    ),
                                                    ropd: 30,
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        598,
                                                    ),
                                                    opd: 34,
                                                },
                                                Expr::Binary {
                                                    lopd: 35,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        597,
                                                    ),
                                                    ropd: 36,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        606,
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
                                                        607,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        611,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        613,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        609,
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
                                                        612,
                                                    ),
                                                    ropd: 41,
                                                },
                                                Expr::Binary {
                                                    lopd: 42,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        610,
                                                    ),
                                                    ropd: 43,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        615,
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
                                                        623,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 47,
                                                    dot_token_idx: TokenIdx(
                                                        624,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            625,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        629,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        631,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        621,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 48,
                                                    dot_token_idx: TokenIdx(
                                                        626,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `cyclic_slice`,
                                                        token_idx: TokenIdx(
                                                            627,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        628,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        49..51,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        632,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 46,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        620,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        51..53,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            622,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        633,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 45,
                                                    dot_token_idx: TokenIdx(
                                                        616,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            617,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        618,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        53..54,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        634,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        635,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        637,
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
                                                        636,
                                                    ),
                                                    ropd: 56,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        640,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        642,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        638,
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
                                                        641,
                                                    ),
                                                    ropd: 59,
                                                },
                                                Expr::Binary {
                                                    lopd: 60,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        639,
                                                    ),
                                                    ropd: 61,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        644,
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
                                                    token_idx: TokenIdx(
                                                        540,
                                                    ),
                                                    ident: `ConcaveComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        571,
                                                    ),
                                                    ident: `is_convex`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        599,
                                                    ),
                                                    ident: `is_convex`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        619,
                                                    ),
                                                    ident: `ConcaveComponent`,
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
                                                            591,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        37,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    605,
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
                                                                608,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            44,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        614,
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
                                                            534,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonToken(
                                                                        TokenIdx(
                                                                            537,
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
                                                                541,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            544,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 1,
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
                                                                546,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        6,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            554,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 2,
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
                                                                557,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            559,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 3,
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
                                                                562,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        8,
                                                    ),
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            564,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        18,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    577,
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
                                                            580,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 4,
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
                                                                582,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        21,
                                                    ),
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            584,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        26,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    590,
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
                                                            643,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        63,
                                                    ),
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `concave_components`,
                                                            token_idx: TokenIdx(
                                                                536,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `L`,
                                                            token_idx: TokenIdx(
                                                                545,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                556,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                561,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `ccv_start`,
                                                            token_idx: TokenIdx(
                                                                581,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                            ],
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
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `line_segment_sketch`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            537,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    645,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `concave_components`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            546,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    645,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `L`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            557,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    645,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `start`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            562,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    645,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `end`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            582,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    645,
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
                                                LetVariables {
                                                    pattern: 0,
                                                    ty: 2,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr: 64,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    64,
                                ),
                            },
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
                    Defn::Impl(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 75,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 75,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            49,
                                        ),
                                    },
                                    ty_expr: 8,
                                    body: ArenaIdxRange(
                                        39..53,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        49,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            51,
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
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                    ident: `ConcaveComponent`,
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                expr: 0,
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEolColon(
                                TokenIdx(
                                    54,
                                ),
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEolColon(
                                TokenIdx(
                                    63,
                                ),
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEolColon(
                                TokenIdx(
                                    82,
                                ),
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEolColon(
                                TokenIdx(
                                    170,
                                ),
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEolColon(
                                TokenIdx(
                                    242,
                                ),
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEolColon(
                                TokenIdx(
                                    372,
                                ),
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
                            TypeItemDefn::Method(
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
                                                    ast_idx: 75,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    ty_expr: 8,
                                                    body: ArenaIdxRange(
                                                        39..53,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 45,
                                            ident: `line_segment`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 45,
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
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
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
                                                                pattern_infos: [],
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_arena: Arena {
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
                                                                    expr: 0,
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
                                                            token_idx: TokenIdx(
                                                                393,
                                                            ),
                                                            ident: `LineSegment`,
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
                                                    pattern_infos: [],
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_arena: Arena {
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
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    390,
                                                ),
                                            ),
                                            self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    391,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    392,
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
                                                    394,
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
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
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
                                                                        pattern_infos: [],
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_arena: Arena {
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
                                                                            expr: 0,
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
                                                                    token_idx: TokenIdx(
                                                                        393,
                                                                    ),
                                                                    ident: `LineSegment`,
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
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
                                                            397,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            398,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                399,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            400,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                401,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            402,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            403,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            404,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                405,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            411,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 5,
                                                        dot_token_idx: TokenIdx(
                                                            412,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                413,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            414,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `lastx`,
                                                            token_idx: TokenIdx(
                                                                415,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            416,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            7..7,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            417,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            418,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                419,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 4,
                                                        dot_token_idx: TokenIdx(
                                                            406,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                407,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            408,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            5..5,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            409,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 8,
                                                        dot_token_idx: TokenIdx(
                                                            420,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                421,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            422,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            9..9,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            423,
                                                        ),
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 0,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            396,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..11,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                410,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            424,
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
                                                        token_idx: TokenIdx(
                                                            395,
                                                        ),
                                                        ident: `LineSegment`,
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
                                                        expr_idx: 11,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    kind: BlockExpr,
                                                    expr: 12,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        12,
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
                            TypeItemDefn::Method(
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
                                                    ast_idx: 75,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    ty_expr: 8,
                                                    body: ArenaIdxRange(
                                                        39..53,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 46,
                                            ident: `start`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 46,
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
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
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
                                                                pattern_infos: [],
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_arena: Arena {
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
                                                                    expr: 0,
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
                                                            token_idx: TokenIdx(
                                                                430,
                                                            ),
                                                            ident: `Point2d`,
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
                                                    pattern_infos: [],
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_arena: Arena {
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
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    427,
                                                ),
                                            ),
                                            self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    428,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    429,
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
                                                    431,
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
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
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
                                                                        pattern_infos: [],
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_arena: Arena {
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
                                                                            expr: 0,
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
                                                                    token_idx: TokenIdx(
                                                                        430,
                                                                    ),
                                                                    ident: `Point2d`,
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
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
                                                            432,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            433,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                434,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            435,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                436,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            437,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            438,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 2,
                                                        dot_token_idx: TokenIdx(
                                                            439,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                440,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 3,
                                                        dot_token_idx: TokenIdx(
                                                            441,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                442,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            443,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            444,
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    kind: BlockExpr,
                                                    expr: 5,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
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
                        ident: `end`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
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
                                                    ast_idx: 75,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    ty_expr: 8,
                                                    body: ArenaIdxRange(
                                                        39..53,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 47,
                                            ident: `end`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
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
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
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
                                                                pattern_infos: [],
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_arena: Arena {
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
                                                                    expr: 0,
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
                                                            token_idx: TokenIdx(
                                                                450,
                                                            ),
                                                            ident: `Point2d`,
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
                                                    pattern_infos: [],
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_arena: Arena {
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
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    447,
                                                ),
                                            ),
                                            self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    448,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    449,
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
                                                    451,
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
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
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
                                                                        pattern_infos: [],
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_arena: Arena {
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
                                                                            expr: 0,
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
                                                                    token_idx: TokenIdx(
                                                                        450,
                                                                    ),
                                                                    ident: `Point2d`,
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
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
                                                            452,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            453,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                454,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            455,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `lastx`,
                                                            token_idx: TokenIdx(
                                                                456,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            457,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            458,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 2,
                                                        dot_token_idx: TokenIdx(
                                                            459,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                460,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 3,
                                                        dot_token_idx: TokenIdx(
                                                            461,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                462,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            463,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            464,
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    kind: BlockExpr,
                                                    expr: 5,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
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
                        ident: `displacement`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
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
                                                    ast_idx: 75,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    ty_expr: 8,
                                                    body: ArenaIdxRange(
                                                        39..53,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 48,
                                            ident: `displacement`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
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
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
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
                                                                pattern_infos: [],
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_arena: Arena {
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
                                                                    expr: 0,
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
                                                            token_idx: TokenIdx(
                                                                470,
                                                            ),
                                                            ident: `Vector2d`,
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
                                                    pattern_infos: [],
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_arena: Arena {
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
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    467,
                                                ),
                                            ),
                                            self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    468,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    469,
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
                                                    471,
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
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
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
                                                                        pattern_infos: [],
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_arena: Arena {
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
                                                                            expr: 0,
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
                                                                    token_idx: TokenIdx(
                                                                        470,
                                                                    ),
                                                                    ident: `Vector2d`,
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
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
                                                            472,
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 0,
                                                        dot_token_idx: TokenIdx(
                                                            473,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `line_segment`,
                                                            token_idx: TokenIdx(
                                                                474,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            475,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            476,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            477,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                478,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            479,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            480,
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    kind: BlockExpr,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
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
                            TypeItemDefn::Method(
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
                                                    ast_idx: 75,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    ty_expr: 8,
                                                    body: ArenaIdxRange(
                                                        39..53,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 49,
                                            ident: `start_tangent`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
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
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
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
                                                                pattern_infos: [],
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_arena: Arena {
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
                                                                    expr: 0,
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
                                                            token_idx: TokenIdx(
                                                                486,
                                                            ),
                                                            ident: `Vector2d`,
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
                                                    pattern_infos: [],
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_arena: Arena {
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
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    483,
                                                ),
                                            ),
                                            self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    484,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    485,
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
                                                    487,
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
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
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
                                                                        pattern_infos: [],
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_arena: Arena {
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
                                                                            expr: 0,
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
                                                                    token_idx: TokenIdx(
                                                                        486,
                                                                    ),
                                                                    ident: `Vector2d`,
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
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
                                                            488,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            489,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                490,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            491,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                492,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            493,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            494,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            495,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                496,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            497,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            498,
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
                                                        expr_idx: 3,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    kind: BlockExpr,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        4,
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
                            TypeItemDefn::Method(
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
                                                    ast_idx: 75,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    ty_expr: 8,
                                                    body: ArenaIdxRange(
                                                        39..53,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 50,
                                            ident: `end_tangent`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
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
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
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
                                                                pattern_infos: [],
                                                                pattern_symbol_maps: [],
                                                                pattern_symbol_arena: Arena {
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
                                                                    expr: 0,
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
                                                            token_idx: TokenIdx(
                                                                504,
                                                            ),
                                                            ident: `Vector2d`,
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
                                                    pattern_infos: [],
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_arena: Arena {
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
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    501,
                                                ),
                                            ),
                                            self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    502,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    503,
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
                                                    505,
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
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
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
                                                                        pattern_infos: [],
                                                                        pattern_symbol_maps: [],
                                                                        pattern_symbol_arena: Arena {
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
                                                                            expr: 0,
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
                                                                    token_idx: TokenIdx(
                                                                        504,
                                                                    ),
                                                                    ident: `Vector2d`,
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
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
                                                            506,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            507,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                508,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            509,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `lastx`,
                                                            token_idx: TokenIdx(
                                                                510,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            511,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            512,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            513,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                514,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            515,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            516,
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
                                                        expr_idx: 3,
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    kind: BlockExpr,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        4,
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