Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ast_idx: 22,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            13,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                        opd: 1,
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 2,
                                                        argument: 3,
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
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            18,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            5..5,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            19,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            20,
                                                        ),
                                                        opd: 5,
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 6,
                                                        argument: 7,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            14,
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
                                                            21,
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
                                                    kind: FieldType,
                                                    expr: 4,
                                                },
                                                ExprRoot {
                                                    kind: FieldType,
                                                    expr: 8,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            7,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `matches`,
                                                    token_idx: TokenIdx(
                                                        8,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        17,
                                                    ),
                                                ),
                                                ty: 8,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    15,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    22,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            23,
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
                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                    ast_idx: 24,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            155,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            156,
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
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            154,
                                                        ),
                                                        opd: 2,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            161,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            162,
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
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            165,
                                                        ),
                                                        opd: 5,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Bracketed {
                                                        lpar_token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                        item: 6,
                                                        rpar_token_idx: TokenIdx(
                                                            167,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            169,
                                                        ),
                                                        opd: 7,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 8,
                                                        opr: Curry,
                                                        opr_token_idx: TokenIdx(
                                                            168,
                                                        ),
                                                        ropd: 9,
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 4,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            163,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            10..11,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            171,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 3,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            157,
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
                                                            166,
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
                                                            170,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            174,
                                                        ),
                                                        ident: `FermiMatchResult`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                ident: `concave_components`,
                                                                token_idx: TokenIdx(
                                                                    152,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `templates`,
                                                                token_idx: TokenIdx(
                                                                    159,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
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
                                                            `templates`,
                                                            1,
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
                                                                153,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `concave_components`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                160,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `templates`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 3,
                                                    },
                                                    ExplicitParameter {
                                                        pattern: 1,
                                                        ty: 11,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 12,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                151,
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
                                                        153,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                            RegularParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        160,
                                                    ),
                                                ),
                                                ty: 11,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    158,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                172,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                173,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 12,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                175,
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
                                                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    155,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    0..0,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    156,
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
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    154,
                                                                ),
                                                                opd: 2,
                                                            },
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    161,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    4..4,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    162,
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
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    165,
                                                                ),
                                                                opd: 5,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 2,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::Bracketed {
                                                                lpar_token_idx: TokenIdx(
                                                                    164,
                                                                ),
                                                                item: 6,
                                                                rpar_token_idx: TokenIdx(
                                                                    167,
                                                                ),
                                                            },
                                                            Expr::Prefix {
                                                                opr: Option,
                                                                opr_token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                opd: 7,
                                                            },
                                                            Expr::Binary {
                                                                lopd: 8,
                                                                opr: Curry,
                                                                opr_token_idx: TokenIdx(
                                                                    168,
                                                                ),
                                                                ropd: 9,
                                                            },
                                                            Expr::ExplicitApplicationOrRitchieCall {
                                                                function: 4,
                                                                implicit_arguments: None,
                                                                lpar_token_idx: TokenIdx(
                                                                    163,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    10..11,
                                                                ),
                                                                commas: [],
                                                                rpar_token_idx: TokenIdx(
                                                                    171,
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 3,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    157,
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
                                                                    166,
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
                                                                    170,
                                                                ),
                                                                ident: `f32`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    174,
                                                                ),
                                                                ident: `FermiMatchResult`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        ident: `concave_components`,
                                                                        token_idx: TokenIdx(
                                                                            152,
                                                                        ),
                                                                    },
                                                                },
                                                                PatternExpr::Ident {
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `templates`,
                                                                        token_idx: TokenIdx(
                                                                            159,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                            Parameter,
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
                                                                    `templates`,
                                                                    1,
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
                                                                        153,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `concave_components`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        160,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `templates`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 3,
                                                            },
                                                            ExplicitParameter {
                                                                pattern: 1,
                                                                ty: 11,
                                                            },
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr: 12,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        180,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `concave_components`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `collect_refs`,
                                                        token_idx: TokenIdx(
                                                            182,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        183,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        184,
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
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                    opd: 2,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        190,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                    opd: 3,
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 4,
                                                    argument: 5,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        7..7,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `templates`,
                                                    token_idx: TokenIdx(
                                                        200,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        9,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 8,
                                                    dot_token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            202,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        9..9,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        199,
                                                    ),
                                                    ropd: 10,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `templates`,
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        211,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        9,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 12,
                                                    lbox_token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `matches`,
                                                    token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `template`,
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 16,
                                                    dot_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `pop_with_largest_opt_f32`,
                                                        token_idx: TokenIdx(
                                                            219,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 15,
                                                    dot_token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        18..19,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        223,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `matches`,
                                                    token_idx: TokenIdx(
                                                        227,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        229,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 20,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        226,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        21..23,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            228,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..6,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        193,
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
                                                        225,
                                                    ),
                                                    ident: `FermiMatchResult`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            206,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 2,
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
                                                                208,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        14,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 19,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            176,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 0,
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
                                                                179,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            185,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 1,
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonToken(
                                                                        TokenIdx(
                                                                            188,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                194,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::ForBetween {
                                                    for_token: StmtForToken {
                                                        token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            198,
                                                        ),
                                                        frame_var_expr_idx: 9,
                                                        frame_var_ident: `i`,
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: None,
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    10,
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
                                                                    205,
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
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            224,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        23,
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
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                178,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `matches`,
                                                            token_idx: TokenIdx(
                                                                187,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `template`,
                                                            token_idx: TokenIdx(
                                                                207,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                                Let,
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `others`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `matches`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `template`,
                                                        2,
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
                                                            ident: `concave_components`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `templates`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            179,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    231,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            188,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    231,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `matches`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            206,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    224,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            208,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    224,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `template`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                LetVariables {
                                                    pattern: 1,
                                                    ty: 6,
                                                },
                                                FrameVariable,
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr: 24,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    24,
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
                            module_path: `mnist_classifier::fermi`,
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 23,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::fermi`,
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 23,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            24,
                                        ),
                                    },
                                    ty_expr: 11,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                12..15,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            26,
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
                                                        module_path: `mnist_classifier::fermi`,
                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                        25,
                                                    ),
                                                    ident: `FermiMatchResult`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `norm`,
                                            item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ident: `norm`,
                                                item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::fermi`,
                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ident: `norm`,
                                                        item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::fermi`,
                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 23,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                    ty_expr: 11,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                12..15,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 12,
                                            ident: `norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            visibility: Visibility::PubUnder(
                                                `mnist_classifier::fermi`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 12,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    29,
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
                                                31,
                                            ),
                                        ),
                                        expr_or_eol_token: Left(
                                            EolToken::Colon(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                },
                                            ),
                                        ),
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
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            25,
                                                                        ),
                                                                        ident: `FermiMatchResult`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            token_idx: TokenIdx(
                                                                30,
                                                            ),
                                                            ident: `f32`,
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
                                                        kind: VarType,
                                                        expr: 0,
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
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    25,
                                                                                ),
                                                                                ident: `FermiMatchResult`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    token_idx: TokenIdx(
                                                                        30,
                                                                    ),
                                                                    ident: `f32`,
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
                                                                kind: VarType,
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
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            37,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            41,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            42,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            39,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            44,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `ilen`,
                                                            token_idx: TokenIdx(
                                                                45,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            46,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            40,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            55,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                57,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            59,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 8,
                                                        lbox_token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..10,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            60,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 10,
                                                        dot_token_idx: TokenIdx(
                                                            61,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                62,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                53,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            11..12,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            63,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 12,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                        ropd: 13,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            65,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            1..4,
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
                                                        expr_idx: 14,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                33,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern_expr: 0,
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
                                                        initial_value: Ok(
                                                            0,
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                38,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                39,
                                                            ),
                                                            frame_var_expr_idx: 3,
                                                            frame_var_ident: `i`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        4,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 1,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        48,
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
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                64,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            15,
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
                                                                ident: `norm`,
                                                                token_idx: TokenIdx(
                                                                    35,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Let,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `norm`,
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
                                                                36,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        66,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `norm`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                49,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        64,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 3,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 16,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        16,
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
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `rel_norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `rel_norm`,
                                            item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ident: `rel_norm`,
                                                item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::fermi`,
                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `rel_norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ident: `rel_norm`,
                                                        item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::fermi`,
                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 23,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                    ty_expr: 11,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                12..15,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 13,
                                            ident: `rel_norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            visibility: Visibility::PubUnder(
                                                `mnist_classifier::fermi`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 13,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    68,
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
                                                70,
                                            ),
                                        ),
                                        expr_or_eol_token: Left(
                                            EolToken::Colon(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                },
                                            ),
                                        ),
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
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            25,
                                                                        ),
                                                                        ident: `FermiMatchResult`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            token_idx: TokenIdx(
                                                                69,
                                                            ),
                                                            ident: `f32`,
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
                                                        kind: VarType,
                                                        expr: 0,
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
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    25,
                                                                                ),
                                                                                ident: `FermiMatchResult`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    token_idx: TokenIdx(
                                                                        69,
                                                                    ),
                                                                    ident: `f32`,
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
                                                                kind: VarType,
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
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `rel_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            76,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            80,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            81,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                82,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            78,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            83,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `ilen`,
                                                            token_idx: TokenIdx(
                                                                84,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            85,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            86,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            79,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            94,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            95,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                96,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            98,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 8,
                                                        lbox_token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..10,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            99,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 10,
                                                        dot_token_idx: TokenIdx(
                                                            100,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `rel_norm`,
                                                            token_idx: TokenIdx(
                                                                101,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            88,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            91,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                92,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            93,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            11..12,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            102,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 12,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            89,
                                                        ),
                                                        ropd: 13,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            104,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            1..4,
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
                                                        expr_idx: 14,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                72,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern_expr: 0,
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
                                                                    75,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            0,
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                77,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                78,
                                                            ),
                                                            frame_var_expr_idx: 3,
                                                            frame_var_ident: `i`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        4,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 1,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        87,
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
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                103,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            15,
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
                                                                ident: `norm`,
                                                                token_idx: TokenIdx(
                                                                    74,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Let,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `norm`,
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
                                                                75,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        105,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `norm`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                88,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        103,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 3,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 16,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        16,
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
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `angle_change_norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `angle_change_norm`,
                                            item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ident: `angle_change_norm`,
                                                item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::fermi`,
                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `angle_change_norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ident: `angle_change_norm`,
                                                        item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::fermi`,
                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 23,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                    ty_expr: 11,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                12..15,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 14,
                                            ident: `angle_change_norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            visibility: Visibility::PubUnder(
                                                `mnist_classifier::fermi`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 14,
                                        colon_token: Some(
                                            ColonToken(
                                                TokenIdx(
                                                    107,
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
                                                109,
                                            ),
                                        ),
                                        expr_or_eol_token: Left(
                                            EolToken::Colon(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        110,
                                                    ),
                                                },
                                            ),
                                        ),
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
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            25,
                                                                        ),
                                                                        ident: `FermiMatchResult`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `angle_change_norm`,
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
                                                            token_idx: TokenIdx(
                                                                108,
                                                            ),
                                                            ident: `f32`,
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
                                                        kind: VarType,
                                                        expr: 0,
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
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                    25,
                                                                                ),
                                                                                ident: `FermiMatchResult`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `angle_change_norm`,
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
                                                                    token_idx: TokenIdx(
                                                                        108,
                                                                    ),
                                                                    ident: `f32`,
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
                                                                kind: VarType,
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
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `angle_change_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            115,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            119,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            120,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                121,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            122,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `ilen`,
                                                            token_idx: TokenIdx(
                                                                123,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            125,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            118,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            129,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            133,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            134,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                135,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            137,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 8,
                                                        lbox_token_idx: TokenIdx(
                                                            136,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..10,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 10,
                                                        dot_token_idx: TokenIdx(
                                                            139,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `angle_change`,
                                                            token_idx: TokenIdx(
                                                                140,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 11,
                                                        dot_token_idx: TokenIdx(
                                                            141,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `abs`,
                                                            token_idx: TokenIdx(
                                                                142,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            143,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            12..12,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            144,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            127,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                131,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            132,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            12..13,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            145,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 13,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            128,
                                                        ),
                                                        ropd: 14,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            147,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            1..4,
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
                                                        expr_idx: 15,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                111,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern_expr: 0,
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
                                                                    114,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            0,
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                116,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                117,
                                                            ),
                                                            frame_var_expr_idx: 3,
                                                            frame_var_ident: `i`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        4,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 1,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        126,
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
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                146,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            16,
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
                                                                ident: `norm`,
                                                                token_idx: TokenIdx(
                                                                    113,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Let,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `norm`,
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
                                                                114,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        148,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `norm`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                127,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        146,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 3,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 17,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        17,
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