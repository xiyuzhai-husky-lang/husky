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
                                                                        value: 201,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: RegularStructFieldType {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 409,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 8,
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
                                            RegularStructFieldDeclPattern {
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
                                                ty_expr_idx: 4,
                                                initialization: None,
                                            },
                                            RegularStructFieldDeclPattern {
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
                                                ty_expr_idx: 8,
                                                initialization: None,
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
                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                    ast_idx: 24,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            152,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            153,
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
                                                            151,
                                                        ),
                                                        opd: 2,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            158,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            159,
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
                                                            163,
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
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            167,
                                                        ),
                                                        opd: 7,
                                                    },
                                                    Expr::Ritchie {
                                                        ritchie_kind_token_idx: TokenIdx(
                                                            161,
                                                        ),
                                                        ritchie_kind: FnType,
                                                        lpar_token: LeftParenthesisToken(
                                                            TokenIdx(
                                                                162,
                                                            ),
                                                        ),
                                                        parameter_ty_exprs: ArenaIdxRange(
                                                            6..7,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            165,
                                                        ),
                                                        light_arrow_token: Some(
                                                            LightArrowToken(
                                                                TokenIdx(
                                                                    166,
                                                                ),
                                                            ),
                                                        ),
                                                        return_ty_expr: Some(
                                                            8,
                                                        ),
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 4,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            160,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..10,
                                                        ),
                                                        commas: [],
                                                        rpar_token_idx: TokenIdx(
                                                            169,
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
                                                            154,
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
                                                            164,
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
                                                            168,
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
                                                            172,
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `concave_components`,
                                                                token_idx: TokenIdx(
                                                                    149,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `templates`,
                                                                token_idx: TokenIdx(
                                                                    156,
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
                                                                150,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `concave_components`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                157,
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
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 3,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 1,
                                                            ty: 10,
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
                                                    expr_idx: 3,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 10,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 11,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                148,
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
                                                        150,
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
                                                        157,
                                                    ),
                                                ),
                                                ty: 10,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    155,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                170,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                171,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 11,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                173,
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
                                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    152,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    0..0,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    153,
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
                                                                    151,
                                                                ),
                                                                opd: 2,
                                                            },
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    158,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    4..4,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    159,
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
                                                                    163,
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
                                                            Expr::Prefix {
                                                                opr: Option,
                                                                opr_token_idx: TokenIdx(
                                                                    167,
                                                                ),
                                                                opd: 7,
                                                            },
                                                            Expr::Ritchie {
                                                                ritchie_kind_token_idx: TokenIdx(
                                                                    161,
                                                                ),
                                                                ritchie_kind: FnType,
                                                                lpar_token: LeftParenthesisToken(
                                                                    TokenIdx(
                                                                        162,
                                                                    ),
                                                                ),
                                                                parameter_ty_exprs: ArenaIdxRange(
                                                                    6..7,
                                                                ),
                                                                commas: [],
                                                                rpar_token_idx: TokenIdx(
                                                                    165,
                                                                ),
                                                                light_arrow_token: Some(
                                                                    LightArrowToken(
                                                                        TokenIdx(
                                                                            166,
                                                                        ),
                                                                    ),
                                                                ),
                                                                return_ty_expr: Some(
                                                                    8,
                                                                ),
                                                            },
                                                            Expr::ExplicitApplicationOrRitchieCall {
                                                                function: 4,
                                                                implicit_arguments: None,
                                                                lpar_token_idx: TokenIdx(
                                                                    160,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    9..10,
                                                                ),
                                                                commas: [],
                                                                rpar_token_idx: TokenIdx(
                                                                    169,
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
                                                                    154,
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
                                                                    164,
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
                                                                    168,
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
                                                                    172,
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
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `concave_components`,
                                                                        token_idx: TokenIdx(
                                                                            149,
                                                                        ),
                                                                    },
                                                                },
                                                                PatternExpr::Ident {
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `templates`,
                                                                        token_idx: TokenIdx(
                                                                            156,
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
                                                                        150,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `concave_components`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    modifier: Pure,
                                                                    access_start: TokenIdx(
                                                                        157,
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
                                                            (
                                                                ExplicitParameter {
                                                                    pattern_expr: 0,
                                                                    ty: 3,
                                                                },
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                            (
                                                                ExplicitParameter {
                                                                    pattern_expr: 1,
                                                                    ty: 10,
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
                                                            expr_idx: 3,
                                                        },
                                                        ExprRoot {
                                                            kind: ExplicitParameterType,
                                                            expr_idx: 10,
                                                        },
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 11,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        178,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `concave_components`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `collect_refs`,
                                                        token_idx: TokenIdx(
                                                            180,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        182,
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
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 2,
                                                    lbox_token_idx: TokenIdx(
                                                        193,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        190,
                                                    ),
                                                    opd: 3,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        187,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                    opd: 4,
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 5,
                                                    argument: 6,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        ExpectedInitialValue(
                                                            TokenStreamState {
                                                                next_token_idx: TokenIdx(
                                                                    195,
                                                                ),
                                                                drained: true,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                Expr::InheritedSymbol {
                                                    ident: `templates`,
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        10,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 9,
                                                    dot_token_idx: TokenIdx(
                                                        199,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            200,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        10..10,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        202,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 10,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        197,
                                                    ),
                                                    ropd: 11,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `templates`,
                                                    token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        10,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 13,
                                                    lbox_token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `matches`,
                                                    token_idx: TokenIdx(
                                                        211,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `template`,
                                                    token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 17,
                                                    dot_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `pop_with_largest_opt_f32`,
                                                        token_idx: TokenIdx(
                                                            217,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        18..19,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 16,
                                                    dot_token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            213,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        19..20,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        221,
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
                                                        225,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        227,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 21,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        224,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        22..24,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            226,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        228,
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
                                                        191,
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
                                                        223,
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
                                                            204,
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
                                                                206,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 15,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 20,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            174,
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
                                                                177,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 1,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            183,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 1,
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonToken(
                                                                        TokenIdx(
                                                                            186,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        ExprError::Original(
                                                            ExpectedAssign(
                                                                TokenStreamState {
                                                                    next_token_idx: TokenIdx(
                                                                        195,
                                                                    ),
                                                                    drained: true,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 8,
                                                },
                                                Stmt::ForBetween {
                                                    for_token: StmtForToken {
                                                        token_idx: TokenIdx(
                                                            195,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            196,
                                                        ),
                                                        frame_var_expr_idx: 10,
                                                        frame_var_ident: `i`,
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: None,
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    11,
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
                                                                    203,
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
                                                            222,
                                                        ),
                                                    },
                                                    result: 24,
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
                                                                        175,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                176,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        184,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `matches`,
                                                            token_idx: TokenIdx(
                                                                185,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `template`,
                                                            token_idx: TokenIdx(
                                                                205,
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
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Pure,
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `concave_components`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        modifier: Pure,
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `templates`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            177,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            186,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `matches`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            204,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            206,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    222,
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
                                                (
                                                    LetVariables {
                                                        pattern: 1,
                                                        ty: 7,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
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
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 7,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 8,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 15,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 20,
                                            },
                                            ExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 24,
                                            },
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 25,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    25,
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
                    Defn::ImplBlock(
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
                                    ty_expr: 23,
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
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `norm`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        32,
                                    ),
                                    drained: true,
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        70,
                                    ),
                                    drained: true,
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
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        108,
                                    ),
                                    drained: true,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)