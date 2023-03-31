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
                                                            12,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            9,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            11,
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
                                                            17,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            5..5,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            19,
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
                                                            13,
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
                                                            20,
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
                                            6,
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
                                                        7,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        8,
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
                                                        15,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        16,
                                                    ),
                                                ),
                                                ty: 8,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    14,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    21,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            22,
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
                                                            151,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            152,
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
                                                            150,
                                                        ),
                                                        opd: 2,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            157,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            158,
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
                                                            161,
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
                                                            160,
                                                        ),
                                                        item: 6,
                                                        rpar_token_idx: TokenIdx(
                                                            163,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            165,
                                                        ),
                                                        opd: 7,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 8,
                                                        opr: Curry,
                                                        opr_token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                        ropd: 9,
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 4,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            159,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            10..11,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            167,
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
                                                            153,
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
                                                            162,
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
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            170,
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
                                                                    148,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `templates`,
                                                                token_idx: TokenIdx(
                                                                    155,
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
                                                                149,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `concave_components`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                156,
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
                                                147,
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
                                                        149,
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
                                                        156,
                                                    ),
                                                ),
                                                ty: 11,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    154,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                168,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                169,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 12,
                                        },
                                    ),
                                    eol_colon: EolColonToken(
                                        TokenIdx(
                                            171,
                                        ),
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
                                                                    151,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    0..0,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    152,
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
                                                                    150,
                                                                ),
                                                                opd: 2,
                                                            },
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    157,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    4..4,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    158,
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
                                                                    161,
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
                                                                    160,
                                                                ),
                                                                item: 6,
                                                                rpar_token_idx: TokenIdx(
                                                                    163,
                                                                ),
                                                            },
                                                            Expr::Prefix {
                                                                opr: Option,
                                                                opr_token_idx: TokenIdx(
                                                                    165,
                                                                ),
                                                                opd: 7,
                                                            },
                                                            Expr::Binary {
                                                                lopd: 8,
                                                                opr: Curry,
                                                                opr_token_idx: TokenIdx(
                                                                    164,
                                                                ),
                                                                ropd: 9,
                                                            },
                                                            Expr::ExplicitApplicationOrRitchieCall {
                                                                function: 4,
                                                                implicit_arguments: None,
                                                                lpar_token_idx: TokenIdx(
                                                                    159,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    10..11,
                                                                ),
                                                                commas: [],
                                                                rpar_token_idx: TokenIdx(
                                                                    167,
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
                                                                    153,
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
                                                                    162,
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
                                                                ident: `f32`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    170,
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
                                                                            148,
                                                                        ),
                                                                    },
                                                                },
                                                                PatternExpr::Ident {
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `templates`,
                                                                        token_idx: TokenIdx(
                                                                            155,
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
                                                                        149,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `concave_components`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        156,
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
                                                        176,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `concave_components`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `collect_refs`,
                                                        token_idx: TokenIdx(
                                                            178,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        180,
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
                                                        188,
                                                    ),
                                                    opd: 2,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        187,
                                                    ),
                                                    opd: 3,
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 4,
                                                    argument: 5,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        7..7,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `templates`,
                                                    token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        194,
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
                                                        197,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            198,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        199,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        9..9,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        200,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                    ropd: 10,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `templates`,
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        9,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 12,
                                                    lbox_token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `matches`,
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `template`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 16,
                                                    dot_token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `pop_with_largest_opt_f32`,
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 15,
                                                    dot_token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            211,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        18..19,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        219,
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
                                                        223,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        225,
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
                                                        222,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        21..23,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        226,
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
                                                        189,
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
                                                        221,
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
                                                            202,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 2,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                204,
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
                                                            172,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 0,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                175,
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
                                                            181,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 1,
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonToken(
                                                                        TokenIdx(
                                                                            184,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                190,
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
                                                            193,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            194,
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
                                                        EolColonToken(
                                                            TokenIdx(
                                                                201,
                                                            ),
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
                                                            220,
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
                                                                174,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `matches`,
                                                            token_idx: TokenIdx(
                                                                183,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `template`,
                                                            token_idx: TokenIdx(
                                                                203,
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
                                                            175,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
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
                                                            184,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
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
                                                            202,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    220,
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
                                                            204,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    220,
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
                                body: Ok(
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
                                            23,
                                        ),
                                    },
                                    ty_expr: 11,
                                    body: ArenaIdxRange(
                                        12..15,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolColonToken(
                                    TokenIdx(
                                        25,
                                    ),
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
                                                        24,
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
                                                            23,
                                                        ),
                                                    },
                                                    ty_expr: 11,
                                                    body: ArenaIdxRange(
                                                        12..15,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 12,
                                            ident: `norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `mnist_classifier::fermi`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 12,
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
                                                                            24,
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
                                                                29,
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
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    28,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolColonToken(
                                            TokenIdx(
                                                30,
                                            ),
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
                                                                                    24,
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
                                                                        29,
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
                                                            35,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            39,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            40,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                41,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            37,
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
                                                            42,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `ilen`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            44,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                        ropd: 4,
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
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            53,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                55,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            57,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 8,
                                                        lbox_token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..10,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 10,
                                                        dot_token_idx: TokenIdx(
                                                            59,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                60,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            47,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                51,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            11..12,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            61,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 12,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            48,
                                                        ),
                                                        ropd: 13,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            63,
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
                                                                31,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern: 0,
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
                                                            AssignToken(
                                                                TokenIdx(
                                                                    34,
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
                                                                36,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                37,
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
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    46,
                                                                ),
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
                                                                62,
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
                                                                    33,
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
                                                                34,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        64,
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
                                                                47,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        62,
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
                                    body: Ok(
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
                                                            23,
                                                        ),
                                                    },
                                                    ty_expr: 11,
                                                    body: ArenaIdxRange(
                                                        12..15,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 13,
                                            ident: `rel_norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `mnist_classifier::fermi`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 13,
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
                                                                            24,
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
                                                                67,
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
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    66,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolColonToken(
                                            TokenIdx(
                                                68,
                                            ),
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
                                                                                    24,
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
                                                                        67,
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
                                                            73,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            77,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            78,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                79,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            75,
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
                                                            80,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `ilen`,
                                                            token_idx: TokenIdx(
                                                                81,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            82,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            83,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            76,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            87,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            91,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                93,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            95,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 8,
                                                        lbox_token_idx: TokenIdx(
                                                            94,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..10,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 10,
                                                        dot_token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `rel_norm`,
                                                            token_idx: TokenIdx(
                                                                98,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            85,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            88,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                89,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            11..12,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            99,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 12,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            86,
                                                        ),
                                                        ropd: 13,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            101,
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
                                                                69,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern: 0,
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
                                                            AssignToken(
                                                                TokenIdx(
                                                                    72,
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
                                                                74,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                75,
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
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    84,
                                                                ),
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
                                                                100,
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
                                                                    71,
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
                                                                72,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        102,
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
                                                                85,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        100,
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
                                    body: Ok(
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
                                                            23,
                                                        ),
                                                    },
                                                    ty_expr: 11,
                                                    body: ArenaIdxRange(
                                                        12..15,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 14,
                                            ident: `angle_change_norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Visibility::PublicUnder(
                                                `mnist_classifier::fermi`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 14,
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
                                                                            24,
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
                                                                105,
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
                                                        kind: ReturnType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    104,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolColonToken(
                                            TokenIdx(
                                                106,
                                            ),
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
                                                                                    24,
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
                                                                        105,
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
                                                            111,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            115,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            116,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                117,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            113,
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
                                                            118,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `ilen`,
                                                            token_idx: TokenIdx(
                                                                119,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            120,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            121,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            125,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            129,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                131,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            133,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            3,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 8,
                                                        lbox_token_idx: TokenIdx(
                                                            132,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..10,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            134,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 10,
                                                        dot_token_idx: TokenIdx(
                                                            135,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `angle_change`,
                                                            token_idx: TokenIdx(
                                                                136,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 11,
                                                        dot_token_idx: TokenIdx(
                                                            137,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `abs`,
                                                            token_idx: TokenIdx(
                                                                138,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            139,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            12..12,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            140,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                127,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            128,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            12..13,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            141,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 13,
                                                        opr: Assign,
                                                        opr_token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                        ropd: 14,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            143,
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
                                                                107,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern: 0,
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
                                                            AssignToken(
                                                                TokenIdx(
                                                                    110,
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
                                                                112,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                113,
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
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    122,
                                                                ),
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
                                                                142,
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
                                                                    109,
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
                                                                110,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        144,
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
                                                                123,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        142,
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
                                    body: Ok(
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