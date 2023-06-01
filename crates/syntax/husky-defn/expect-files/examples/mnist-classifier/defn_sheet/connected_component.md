Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    ast_idx: 120,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                                    TypePath(`core::num::i32`, `Extern`),
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
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 3,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
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
                                                            16,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            20,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            28,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
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
                                                                        value: 196,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 0,
                                                },
                                                ExprRoot {
                                                    kind: RegularStructFieldType {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 197,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                18,
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
                                                                        value: 198,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                22,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 2,
                                                },
                                                ExprRoot {
                                                    kind: RegularStructFieldType {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 199,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                26,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            13,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            RegularStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `row_start`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        15,
                                                    ),
                                                ),
                                                ty_expr_idx: 0,
                                                initialization: None,
                                            },
                                            RegularStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `row_end`,
                                                    token_idx: TokenIdx(
                                                        18,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        19,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                            RegularStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `upper_mass`,
                                                    token_idx: TokenIdx(
                                                        22,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        23,
                                                    ),
                                                ),
                                                ty_expr_idx: 2,
                                                initialization: None,
                                            },
                                            RegularStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `lower_mass`,
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        27,
                                                    ),
                                                ),
                                                ty_expr_idx: 3,
                                                initialization: None,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    17,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    21,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    25,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    29,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            30,
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
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    ast_idx: 121,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            40,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            37,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            39,
                                                        ),
                                                        opd: 1,
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
                                                            41,
                                                        ),
                                                        ident: `RawContour`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                35,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 4,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            34,
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
                                                        35,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        36,
                                                    ),
                                                ),
                                                ty_expr_idx: 4,
                                                initialization: None,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    42,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            43,
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
                            FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                    ast_idx: 122,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
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
                                                            53,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                        ident: `RawContour`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            54,
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
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `ct`,
                                                                token_idx: TokenIdx(
                                                                    47,
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
                                                            `ct`,
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
                                                                48,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `ct`,
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
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                46,
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
                                                        48,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                51,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                52,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                55,
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
                                                                    FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
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
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    49,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
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
                                                                    53,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: `RawContour`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    54,
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
                                                            data: [
                                                                PatternExpr::Ident {
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `ct`,
                                                                        token_idx: TokenIdx(
                                                                            47,
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
                                                                    `ct`,
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
                                                                        48,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `ct`,
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
                                                            expr_idx: 3,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `ct`,
                                                    token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `ct`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 0,
                                                    dot_token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `contour_len`,
                                                        token_idx: TokenIdx(
                                                            61,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `len`,
                                                    token_idx: TokenIdx(
                                                        63,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        65,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 2,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                    ropd: 3,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `len`,
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        68,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 5,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        67,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        0..3,
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
                                                            56,
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
                                                                58,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 1,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                    condition: 4,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 7,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `len`,
                                                            token_idx: TokenIdx(
                                                                57,
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
                                                Let,
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
                                                        `len`,
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
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Pure,
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `ct`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            58,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    69,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `len`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: Condition,
                                                expr_idx: 4,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 7,
                                            },
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 8,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    8,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    ast_idx: 123,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                    TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                                            75,
                                                        ),
                                                        ident: `BinaryImage28`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                                                        value: 207,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                73,
                                                            ),
                                                        },
                                                    },
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            72,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            RegularStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        74,
                                                    ),
                                                ),
                                                ty_expr_idx: 0,
                                                initialization: None,
                                            },
                                        ],
                                        [],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            76,
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
                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                    ast_idx: 126,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
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
                                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                            572,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            576,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            579,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                                ident: `a`,
                                                                token_idx: TokenIdx(
                                                                    570,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `x`,
                                                                token_idx: TokenIdx(
                                                                    574,
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
                                                            `a`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `x`,
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
                                                                571,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `a`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                575,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `x`,
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
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                569,
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
                                                        571,
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
                                                        575,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    573,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                577,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                578,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 2,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                580,
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
                                                                    FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
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
                                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 2,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                                    572,
                                                                ),
                                                                ident: `r32`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::raw_bits::r32`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    576,
                                                                ),
                                                                ident: `r32`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::raw_bits::r32`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    579,
                                                                ),
                                                                ident: `r32`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                                        ident: `a`,
                                                                        token_idx: TokenIdx(
                                                                            570,
                                                                        ),
                                                                    },
                                                                },
                                                                PatternExpr::Ident {
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `x`,
                                                                        token_idx: TokenIdx(
                                                                            574,
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
                                                                    `a`,
                                                                    0,
                                                                ),
                                                            ],
                                                            [
                                                                (
                                                                    `x`,
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
                                                                        571,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `a`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    modifier: Pure,
                                                                    access_start: TokenIdx(
                                                                        575,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `x`,
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
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        591,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `x`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        593,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 0,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        592,
                                                    ),
                                                    ropd: 1,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        588,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `x`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        590,
                                                    ),
                                                    item: 2,
                                                    rpar_token_idx: TokenIdx(
                                                        594,
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        597,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `x`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        599,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 5,
                                                    opr: Shift(
                                                        Shr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        598,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        589,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        596,
                                                    ),
                                                    item: 7,
                                                    rpar_token_idx: TokenIdx(
                                                        600,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 8,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        595,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        585,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `a`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        587,
                                                    ),
                                                    item: 10,
                                                    rpar_token_idx: TokenIdx(
                                                        601,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 11,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        586,
                                                    ),
                                                    ropd: 12,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        612,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        614,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 14,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        613,
                                                    ),
                                                    ropd: 15,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        609,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        611,
                                                    ),
                                                    item: 16,
                                                    rpar_token_idx: TokenIdx(
                                                        615,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        618,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        620,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 19,
                                                    opr: Shift(
                                                        Shr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        619,
                                                    ),
                                                    ropd: 20,
                                                },
                                                Expr::Binary {
                                                    lopd: 17,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        610,
                                                    ),
                                                    ropd: 18,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        617,
                                                    ),
                                                    item: 21,
                                                    rpar_token_idx: TokenIdx(
                                                        621,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 22,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        616,
                                                    ),
                                                    ropd: 23,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        606,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `a`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        608,
                                                    ),
                                                    item: 24,
                                                    rpar_token_idx: TokenIdx(
                                                        622,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 25,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        607,
                                                    ),
                                                    ropd: 26,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `z`,
                                                    token_idx: TokenIdx(
                                                        624,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        626,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 28,
                                                    opr: Comparison(
                                                        Neq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        625,
                                                    ),
                                                    ropd: 29,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        628,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `z`,
                                                    token_idx: TokenIdx(
                                                        630,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 31,
                                                    argument: 32,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        639,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        641,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 34,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        640,
                                                    ),
                                                    ropd: 35,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        636,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        638,
                                                    ),
                                                    item: 36,
                                                    rpar_token_idx: TokenIdx(
                                                        642,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        645,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        647,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 39,
                                                    opr: Shift(
                                                        Shr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        646,
                                                    ),
                                                    ropd: 40,
                                                },
                                                Expr::Binary {
                                                    lopd: 37,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        637,
                                                    ),
                                                    ropd: 38,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        644,
                                                    ),
                                                    item: 41,
                                                    rpar_token_idx: TokenIdx(
                                                        648,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 42,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        643,
                                                    ),
                                                    ropd: 43,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        633,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `a`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        635,
                                                    ),
                                                    item: 44,
                                                    rpar_token_idx: TokenIdx(
                                                        649,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `z`,
                                                    token_idx: TokenIdx(
                                                        631,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 45,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        634,
                                                    ),
                                                    ropd: 46,
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 47,
                                                    argument: 48,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        651,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..6,
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
                                                Stmt::Eval {
                                                    expr_idx: 49,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            581,
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
                                                                584,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 13,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            602,
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
                                                                605,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 27,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            623,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        30,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    627,
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
                                                            650,
                                                        ),
                                                    },
                                                    result: 50,
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
                                                                        582,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                583,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        603,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `z`,
                                                            token_idx: TokenIdx(
                                                                604,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    Move,
                                                ],
                                            },
                                            pattern_infos: [
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
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `y`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `z`,
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                    Mut,
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
                                                            ident: `a`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        modifier: Pure,
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `x`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            584,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    652,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `y`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            605,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    652,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `z`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 13,
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
                                                kind: EvalExpr,
                                                expr_idx: 49,
                                            },
                                            ExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 50,
                                            },
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 51,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    51,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                    ast_idx: 127,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
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
                                                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            661,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            662,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            658,
                                                        ),
                                                        ident: `BinaryImage28`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::BinaryImage28`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            663,
                                                        ),
                                                        ident: `ConnectedComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                ident: `img`,
                                                                token_idx: TokenIdx(
                                                                    656,
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
                                                            `img`,
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
                                                                657,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `img`,
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
                                                            ty: 0,
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
                                                    expr_idx: 0,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                655,
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
                                                        657,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                659,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                660,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                664,
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
                                                                    FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
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
                                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    661,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    1..1,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    662,
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::ExplicitApplication {
                                                                function: 1,
                                                                argument: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    658,
                                                                ),
                                                                ident: `BinaryImage28`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    663,
                                                                ),
                                                                ident: `ConnectedComponent`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        ident: `img`,
                                                                        token_idx: TokenIdx(
                                                                            656,
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
                                                                    `img`,
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
                                                                        657,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `img`,
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
                                                                    ty: 0,
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
                                                            expr_idx: 0,
                                                        },
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 3,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
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
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        669,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        670,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 0,
                                                    lbox_token_idx: TokenIdx(
                                                        673,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        674,
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 1,
                                                    argument: 2,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        ExpectedInitialValue(
                                                            TokenStreamState {
                                                                next_token_idx: TokenIdx(
                                                                    675,
                                                                ),
                                                                drained: true,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                Expr::InheritedSymbol {
                                                    ident: `img`,
                                                    token_idx: TokenIdx(
                                                        679,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `img`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 5,
                                                    dot_token_idx: TokenIdx(
                                                        680,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `clone`,
                                                        token_idx: TokenIdx(
                                                            681,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        682,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        6..6,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        683,
                                                    ),
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        685,
                                                    ),
                                                    ident: `j`,
                                                    frame_var_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        7,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        687,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 7,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        686,
                                                    ),
                                                    ropd: 8,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `unsearched`,
                                                    token_idx: TokenIdx(
                                                        690,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        692,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        7,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 10,
                                                    lbox_token_idx: TokenIdx(
                                                        691,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        11..12,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        693,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `unsearched`,
                                                    token_idx: TokenIdx(
                                                        698,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        700,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        7,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 13,
                                                    lbox_token_idx: TokenIdx(
                                                        699,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        701,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        705,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 16,
                                                    dot_token_idx: TokenIdx(
                                                        706,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ctz`,
                                                        token_idx: TokenIdx(
                                                            707,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        708,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        17..17,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        709,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::BinaryImage28`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 18,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        715,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        19..19,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        716,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        717,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        719,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        7,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        726,
                                                    ),
                                                    Literal::Integer(
                                                        R32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `shift`,
                                                    token_idx: TokenIdx(
                                                        728,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        724,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 23,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        727,
                                                    ),
                                                    ropd: 24,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 20,
                                                    lbox_token_idx: TokenIdx(
                                                        718,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        21..22,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        720,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 22,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        723,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        25..27,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            725,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        729,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 27,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        721,
                                                    ),
                                                    ropd: 28,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        734,
                                                    ),
                                                    Literal::Bool(
                                                        False,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        737,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        736,
                                                    ),
                                                    opd: 31,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        739,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        741,
                                                    ),
                                                    Literal::Bool(
                                                        True,
                                                    ),
                                                ),
                                                Expr::ExplicitApplication {
                                                    function: 33,
                                                    argument: 34,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        746,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        7,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        750,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        752,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        748,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 37,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        751,
                                                    ),
                                                    ropd: 38,
                                                },
                                                Expr::Binary {
                                                    lopd: 39,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        749,
                                                    ),
                                                    ropd: 40,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        757,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        759,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        761,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 43,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        760,
                                                    ),
                                                    ropd: 44,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 42,
                                                    lbox_token_idx: TokenIdx(
                                                        758,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        45..46,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        762,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `img`,
                                                    token_idx: TokenIdx(
                                                        770,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `img`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        772,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        774,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 49,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        773,
                                                    ),
                                                    ropd: 50,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        777,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        779,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 48,
                                                    lbox_token_idx: TokenIdx(
                                                        771,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        51..52,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        775,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 52,
                                                    lbox_token_idx: TokenIdx(
                                                        778,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        53..54,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        780,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        766,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 47,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        769,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        54..56,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            776,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        781,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 56,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        767,
                                                    ),
                                                    ropd: 57,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        784,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        783,
                                                    ),
                                                    opd: 59,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        788,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        790,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 61,
                                                    opr: Comparison(
                                                        Neq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        789,
                                                    ),
                                                    ropd: 62,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        792,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        794,
                                                    ),
                                                    Literal::Bool(
                                                        False,
                                                    ),
                                                ),
                                                Expr::ExplicitApplication {
                                                    function: 64,
                                                    argument: 65,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        795,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        797,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        799,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 68,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        798,
                                                    ),
                                                    ropd: 69,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 67,
                                                    lbox_token_idx: TokenIdx(
                                                        796,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        70..71,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        800,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        802,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 71,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        801,
                                                    ),
                                                    ropd: 72,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        804,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        806,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        7,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 74,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        805,
                                                    ),
                                                    ropd: 75,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        811,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        813,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 77,
                                                    lbox_token_idx: TokenIdx(
                                                        812,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        78..79,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        814,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `img`,
                                                    token_idx: TokenIdx(
                                                        822,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `img`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        824,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        827,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        829,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        831,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 84,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        830,
                                                    ),
                                                    ropd: 85,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 81,
                                                    lbox_token_idx: TokenIdx(
                                                        823,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        82..83,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        825,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 83,
                                                    lbox_token_idx: TokenIdx(
                                                        828,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        86..87,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        832,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        818,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 80,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        821,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        87..89,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            826,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        833,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 89,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        819,
                                                    ),
                                                    ropd: 90,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        835,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        837,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 92,
                                                    opr: Comparison(
                                                        Neq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        836,
                                                    ),
                                                    ropd: 93,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        839,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        841,
                                                    ),
                                                    Literal::Bool(
                                                        False,
                                                    ),
                                                ),
                                                Expr::ExplicitApplication {
                                                    function: 95,
                                                    argument: 96,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        842,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        844,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 98,
                                                    lbox_token_idx: TokenIdx(
                                                        843,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        99..100,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        845,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        847,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 100,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        846,
                                                    ),
                                                    ropd: 101,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        849,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        7,
                                                    ),
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        851,
                                                    ),
                                                    ident: `k`,
                                                    frame_var_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        104,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 103,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        850,
                                                    ),
                                                    ropd: 104,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        853,
                                                    ),
                                                    Literal::Integer(
                                                        Unspecified,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 105,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        852,
                                                    ),
                                                    ropd: 106,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `unsearched`,
                                                    token_idx: TokenIdx(
                                                        855,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `k`,
                                                    token_idx: TokenIdx(
                                                        857,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        104,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        862,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `k`,
                                                    token_idx: TokenIdx(
                                                        864,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        104,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 110,
                                                    lbox_token_idx: TokenIdx(
                                                        863,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        111..112,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        865,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        861,
                                                    ),
                                                    opd: 112,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 108,
                                                    lbox_token_idx: TokenIdx(
                                                        856,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        109..110,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        858,
                                                    ),
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        860,
                                                    ),
                                                    item: 113,
                                                    rpar_token_idx: TokenIdx(
                                                        866,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 114,
                                                    opr: AssignClosed(
                                                        BitAnd,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        859,
                                                    ),
                                                    ropd: 115,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `result`,
                                                    token_idx: TokenIdx(
                                                        867,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        873,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 118,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        872,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        119..120,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        874,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 117,
                                                    dot_token_idx: TokenIdx(
                                                        868,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            869,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        870,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        120..121,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        875,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `result`,
                                                    token_idx: TokenIdx(
                                                        877,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        26..30,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        671,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        714,
                                                    ),
                                                    ident: `BinaryImage28`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        722,
                                                    ),
                                                    ident: `horizontal_extend`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        768,
                                                    ),
                                                    ident: `horizontal_extend`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        820,
                                                    ),
                                                    ident: `horizontal_extend`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        871,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Break {
                                                    break_token: BreakToken {
                                                        token_idx: TokenIdx(
                                                            786,
                                                        ),
                                                    },
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 66,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 73,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            754,
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
                                                                756,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 46,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            763,
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
                                                                765,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 58,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                782,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            60,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        785,
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
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                787,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            63,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        791,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                1..3,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 97,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 102,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            808,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 9,
                                                            variables: ArenaIdxRange(
                                                                10..11,
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
                                                                810,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 79,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            815,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 10,
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
                                                                817,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 91,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                834,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            94,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        838,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                7..9,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 35,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            742,
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
                                                                745,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 36,
                                                },
                                                Stmt::ForExt {
                                                    forext_token: ForextToken {
                                                        token_idx: TokenIdx(
                                                            747,
                                                        ),
                                                    },
                                                    expr: 41,
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    753,
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
                                                Stmt::ForExt {
                                                    forext_token: ForextToken {
                                                        token_idx: TokenIdx(
                                                            803,
                                                        ),
                                                    },
                                                    expr: 76,
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    807,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            9..12,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 116,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            695,
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
                                                                697,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 15,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            702,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 3,
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
                                                                704,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 17,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            710,
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
                                                                713,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 19,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 29,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            730,
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
                                                                733,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 30,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            735,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        32,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    738,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            12..16,
                                                        ),
                                                    ),
                                                },
                                                Stmt::ForBetween {
                                                    for_token: StmtForToken {
                                                        token_idx: TokenIdx(
                                                            848,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            851,
                                                        ),
                                                        frame_var_expr_idx: 104,
                                                        frame_var_ident: `k`,
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    103,
                                                                ),
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    106,
                                                                ),
                                                                kind: UpperOpen,
                                                            },
                                                            step: Constant(
                                                                1,
                                                            ),
                                                        },
                                                    },
                                                    frame_var_symbol_idx: 12,
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    854,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            16..17,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 121,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            689,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        12,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    694,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            17..25,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            665,
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
                                                                            668,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                3,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        ExprError::Original(
                                                            ExpectedAssign(
                                                                TokenStreamState {
                                                                    next_token_idx: TokenIdx(
                                                                        675,
                                                                    ),
                                                                    drained: true,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 4,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            675,
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
                                                                678,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 6,
                                                },
                                                Stmt::ForBetween {
                                                    for_token: StmtForToken {
                                                        token_idx: TokenIdx(
                                                            684,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            685,
                                                        ),
                                                        frame_var_expr_idx: 7,
                                                        frame_var_ident: `j`,
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: None,
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    8,
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
                                                                    688,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            25..26,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            876,
                                                        ),
                                                    },
                                                    result: 122,
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
                                                                        666,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `result`,
                                                            token_idx: TokenIdx(
                                                                667,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        676,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `unsearched`,
                                                            token_idx: TokenIdx(
                                                                677,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                696,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `shift`,
                                                            token_idx: TokenIdx(
                                                                703,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        711,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                712,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        731,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `flag`,
                                                            token_idx: TokenIdx(
                                                                732,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        743,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `i`,
                                                            token_idx: TokenIdx(
                                                                744,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `old_row`,
                                                            token_idx: TokenIdx(
                                                                755,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `new_row`,
                                                            token_idx: TokenIdx(
                                                                764,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `old_row`,
                                                            token_idx: TokenIdx(
                                                                809,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `new_row`,
                                                            token_idx: TokenIdx(
                                                                816,
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
                                                    Pure,
                                                    Move,
                                                    Move,
                                                    Move,
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
                                                    PatternSymbol::Atom(
                                                        10,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `result`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `unsearched`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `shift`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `mask`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `flag`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `i`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `old_row`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `new_row`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `old_row`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `new_row`,
                                                        10,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                    Mut,
                                                    Pure,
                                                    Pure,
                                                    Mut,
                                                    Mut,
                                                    Mut,
                                                    Pure,
                                                    Pure,
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
                                                            ident: `img`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            668,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    878,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `result`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            678,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    878,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `unsearched`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            689,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    876,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `j`,
                                                            expr_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            697,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    876,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            704,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    876,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `shift`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            713,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    876,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `mask`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            733,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    876,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `flag`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            745,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    848,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `i`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            756,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    803,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `old_row`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            765,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    803,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `new_row`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            810,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    848,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `old_row`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            817,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    848,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `new_row`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            855,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    867,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `k`,
                                                            expr_idx: 104,
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
                                                        ty: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        12..13,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 3,
                                            },
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
                                                expr_idx: 15,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 17,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 19,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 29,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 30,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 35,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 36,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 46,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 58,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 66,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 73,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 79,
                                            },
                                            ExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 91,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 97,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 102,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 116,
                                            },
                                            ExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 121,
                                            },
                                            ExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 122,
                                            },
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 123,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    123,
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
                            module_path: `mnist_classifier::connected_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::ImplBlock(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 124,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `mnist_classifier::connected_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 124,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            77,
                                        ),
                                    },
                                    trai_expr: 0,
                                    for_token: TokenIdx(
                                        79,
                                    ),
                                    ty_expr: 1,
                                    items: Some(
                                        TraitForType(
                                            TraitForTypeItems {
                                                ast_idx_range: ArenaIdxRange(
                                                    4..5,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        77,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        79,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            81,
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
                                                        module_path: `mnist_classifier::connected_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                        78,
                                                    ),
                                                    ident: `Visualize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                module_path: `mnist_classifier::connected_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            trai: TraitPath(`core::visual::Visualize`),
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    decl: TraitForTypeMethodFnDecl {
                                        path: Some(
                                            TraitForTypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                trai: TraitPath(`core::visual::Visualize`),
                                                ident: `visualize`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::TraitForType(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `visualize`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        trai: TraitPath(`core::visual::Visualize`),
                                                        ident: `visualize`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::TraitForType(
                                                TraitForTypeImplBlock {
                                                    id: TraitForTypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 124,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            77,
                                                        ),
                                                    },
                                                    trai_expr: 0,
                                                    for_token: TokenIdx(
                                                        79,
                                                    ),
                                                    ty_expr: 1,
                                                    items: Some(
                                                        TraitForType(
                                                            TraitForTypeItems {
                                                                ast_idx_range: ArenaIdxRange(
                                                                    4..5,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            ast_idx: 4,
                                            ident: `visualize`,
                                            associated_item_kind: TraitForTypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::connected_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 4,
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            78,
                                                                        ),
                                                                        ident: `Visualize`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::visual::Visualize`),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            80,
                                                                        ),
                                                                        ident: `ConnectedComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                            token_idx: TokenIdx(
                                                                87,
                                                            ),
                                                            ident: `Html`,
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
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    84,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    85,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    86,
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
                                                    88,
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
                                                                            ImplBlockId::TraitForType(
                                                                                TraitForTypeImplBlockId {
                                                                                    module_path: `mnist_classifier::connected_component`,
                                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    78,
                                                                                ),
                                                                                ident: `Visualize`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Trait(
                                                                                        TraitPath(`core::visual::Visualize`),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    80,
                                                                                ),
                                                                                ident: `ConnectedComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            trai_path: TraitPath(`core::visual::Visualize`),
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                    token_idx: TokenIdx(
                                                                        87,
                                                                    ),
                                                                    ident: `Html`,
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
                                                                module_path: `mnist_classifier::connected_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                            89,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                91,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `visualize`,
                                                            token_idx: TokenIdx(
                                                                93,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            94,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            95,
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
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::ImplBlock(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 125,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 125,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            96,
                                        ),
                                    },
                                    ty_expr: 2,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                69..79,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        96,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            98,
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
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                        97,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `raw_contours`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        106,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `eff_holes`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        115,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `max_hole_ilen`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        177,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `max_row_span`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        229,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `row_span_sum`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        267,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `distribution`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        300,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `upper_mass`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        422,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `lower_mass`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectEqTokenForVariable(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        434,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `top_k_row_span_sum`,
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
                                                module_path: `mnist_classifier::connected_component`,
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `top_k_row_span_sum`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::connected_component`,
                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `top_k_row_span_sum`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ident: `top_k_row_span_sum`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `top_k_row_span_sum`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ident: `top_k_row_span_sum`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 125,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                    ty_expr: 2,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                69..79,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 77,
                                            ident: `top_k_row_span_sum`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::connected_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 77,
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            97,
                                                                        ),
                                                                        ident: `ConnectedComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `top_k_row_span_sum`,
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
                                                                        TypePath(`core::num::i32`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::EntityPath {
                                                            entity_path_expr: 1,
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
                                                                446,
                                                            ),
                                                            ident: `i32`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                449,
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
                                                        data: [
                                                            PatternExpr::Ident {
                                                                modifier_keyword_group: None,
                                                                ident_token: IdentToken {
                                                                    ident: `k`,
                                                                    token_idx: TokenIdx(
                                                                        444,
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
                                                                `k`,
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
                                                                    445,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ExplicitParameter {
                                                                    ident: `k`,
                                                                    pattern_symbol_idx: 0,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                                    ],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: ExplicitParameterType,
                                                        expr_idx: 0,
                                                    },
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    443,
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
                                                            445,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                            ],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    447,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    448,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 1,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    450,
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
                                                                                    module_path: `mnist_classifier::connected_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    97,
                                                                                ),
                                                                                ident: `ConnectedComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `top_k_row_span_sum`,
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
                                                                                TypePath(`core::num::i32`, `Extern`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 1,
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
                                                                        446,
                                                                    ),
                                                                    ident: `i32`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::i32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        449,
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
                                                                data: [
                                                                    PatternExpr::Ident {
                                                                        modifier_keyword_group: None,
                                                                        ident_token: IdentToken {
                                                                            ident: `k`,
                                                                            token_idx: TokenIdx(
                                                                                444,
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
                                                                        `k`,
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
                                                                            445,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                                            ident: `k`,
                                                                            pattern_symbol_idx: 0,
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                                            ],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ExplicitParameterType,
                                                                expr_idx: 0,
                                                            },
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 1,
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
                                                                module_path: `mnist_classifier::connected_component`,
                                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `top_k_row_span_sum`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            455,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            457,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            459,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 1,
                                                        opr: Comparison(
                                                            Greater,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            458,
                                                        ),
                                                        ropd: 2,
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            464,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            466,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            468,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 5,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            467,
                                                        ),
                                                        ropd: 6,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            471,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 8,
                                                        dot_token_idx: TokenIdx(
                                                            472,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                473,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            475,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 9,
                                                        lbox_token_idx: TokenIdx(
                                                            474,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            10..11,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            476,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            480,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            482,
                                                        ),
                                                        ident: `j`,
                                                        frame_var_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            13,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            484,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            486,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 12,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            481,
                                                        ),
                                                        ropd: 13,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 14,
                                                        opr: Closed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            485,
                                                        ),
                                                        ropd: 15,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 16,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            483,
                                                        ),
                                                        ropd: 17,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            490,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 19,
                                                        dot_token_idx: TokenIdx(
                                                            491,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                492,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            494,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            13,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 20,
                                                        lbox_token_idx: TokenIdx(
                                                            493,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            21..22,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            495,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            488,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 22,
                                                        dot_token_idx: TokenIdx(
                                                            496,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `span`,
                                                            token_idx: TokenIdx(
                                                                497,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            498,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            23..23,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            499,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 23,
                                                        opr: AssignClosed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            489,
                                                        ),
                                                        ropd: 24,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            501,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
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
                                                    Expr::Binary {
                                                        lopd: 26,
                                                        opr: As,
                                                        opr_token_idx: TokenIdx(
                                                            502,
                                                        ),
                                                        ropd: 27,
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            3..9,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            503,
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
                                                data: [
                                                    Stmt::Break {
                                                        break_token: BreakToken {
                                                            token_idx: TokenIdx(
                                                                478,
                                                            ),
                                                        },
                                                    },
                                                    Stmt::IfElse {
                                                        if_branch: IfBranch {
                                                            if_token: IfToken {
                                                                token_idx: TokenIdx(
                                                                    470,
                                                                ),
                                                            },
                                                            condition: Ok(
                                                                11,
                                                            ),
                                                            eol_colon: Ok(
                                                                Colon(
                                                                    EolColonToken {
                                                                        token_idx: TokenIdx(
                                                                            477,
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
                                                    Stmt::Eval {
                                                        expr_idx: 25,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                451,
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
                                                                    454,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 0,
                                                    },
                                                    Stmt::Assert {
                                                        assert_token: AssertToken {
                                                            token_idx: TokenIdx(
                                                                456,
                                                            ),
                                                        },
                                                        condition: 3,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                460,
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
                                                                    463,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 4,
                                                    },
                                                    Stmt::ForExt {
                                                        forext_token: ForextToken {
                                                            token_idx: TokenIdx(
                                                                465,
                                                            ),
                                                        },
                                                        expr: 7,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        469,
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
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                479,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                482,
                                                            ),
                                                            frame_var_expr_idx: 13,
                                                            frame_var_ident: `j`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        12,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        17,
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
                                                                        487,
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
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                500,
                                                            ),
                                                        },
                                                        result: 28,
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
                                                                            452,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `top_k_row_span_sum`,
                                                                token_idx: TokenIdx(
                                                                    453,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            461,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `i`,
                                                                token_idx: TokenIdx(
                                                                    462,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Move,
                                                        Move,
                                                    ],
                                                },
                                                pattern_infos: [
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
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `top_k_row_span_sum`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `i`,
                                                            1,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
                                                        Mut,
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
                                                                ident: `k`,
                                                            },
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                454,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        504,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `top_k_row_span_sum`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                463,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        504,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `i`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                488,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        500,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `j`,
                                                                expr_idx: 13,
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
                                                    kind: Condition,
                                                    expr_idx: 3,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 25,
                                                },
                                                ExprRoot {
                                                    kind: ReturnExpr,
                                                    expr_idx: 28,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 29,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        29,
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
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `top_k_row_right_mass_sum`,
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
                                                module_path: `mnist_classifier::connected_component`,
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `top_k_row_right_mass_sum`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::connected_component`,
                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `top_k_row_right_mass_sum`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ident: `top_k_row_right_mass_sum`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `top_k_row_right_mass_sum`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ident: `top_k_row_right_mass_sum`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 125,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                    ty_expr: 2,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                69..79,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 78,
                                            ident: `top_k_row_right_mass_sum`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::PubUnder(
                                                `mnist_classifier::connected_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 78,
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            97,
                                                                        ),
                                                                        ident: `ConnectedComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `top_k_row_right_mass_sum`,
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
                                                                        TypePath(`core::num::i32`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::EntityPath {
                                                            entity_path_expr: 1,
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
                                                                509,
                                                            ),
                                                            ident: `i32`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                512,
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
                                                        data: [
                                                            PatternExpr::Ident {
                                                                modifier_keyword_group: None,
                                                                ident_token: IdentToken {
                                                                    ident: `k`,
                                                                    token_idx: TokenIdx(
                                                                        507,
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
                                                                `k`,
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
                                                                    508,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ExplicitParameter {
                                                                    ident: `k`,
                                                                    pattern_symbol_idx: 0,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                                    ],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: ExplicitParameterType,
                                                        expr_idx: 0,
                                                    },
                                                    ExprRoot {
                                                        kind: ReturnType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    506,
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
                                                            508,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                            ],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    510,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    511,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 1,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    513,
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
                                                                                    module_path: `mnist_classifier::connected_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    97,
                                                                                ),
                                                                                ident: `ConnectedComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `top_k_row_right_mass_sum`,
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
                                                                                TypePath(`core::num::i32`, `Extern`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 1,
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
                                                                        509,
                                                                    ),
                                                                    ident: `i32`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::i32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        512,
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
                                                                data: [
                                                                    PatternExpr::Ident {
                                                                        modifier_keyword_group: None,
                                                                        ident_token: IdentToken {
                                                                            ident: `k`,
                                                                            token_idx: TokenIdx(
                                                                                507,
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
                                                                        `k`,
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
                                                                            508,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                                            ident: `k`,
                                                                            pattern_symbol_idx: 0,
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                                            ],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: ExplicitParameterType,
                                                                expr_idx: 0,
                                                            },
                                                            ExprRoot {
                                                                kind: ReturnType,
                                                                expr_idx: 1,
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
                                                                module_path: `mnist_classifier::connected_component`,
                                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `top_k_row_right_mass_sum`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            518,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            520,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 1,
                                                        opr: Comparison(
                                                            Greater,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            521,
                                                        ),
                                                        ropd: 2,
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            527,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            529,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            531,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 5,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            530,
                                                        ),
                                                        ropd: 6,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            534,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 8,
                                                        dot_token_idx: TokenIdx(
                                                            535,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                536,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            538,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 9,
                                                        lbox_token_idx: TokenIdx(
                                                            537,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            10..11,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            539,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            543,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            545,
                                                        ),
                                                        ident: `j`,
                                                        frame_var_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            13,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            547,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            549,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 12,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            544,
                                                        ),
                                                        ropd: 13,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 14,
                                                        opr: Closed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            548,
                                                        ),
                                                        ropd: 15,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 16,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            546,
                                                        ),
                                                        ropd: 17,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 19,
                                                        dot_token_idx: TokenIdx(
                                                            554,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                555,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            557,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            13,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 20,
                                                        lbox_token_idx: TokenIdx(
                                                            556,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            21..22,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            558,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            551,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 22,
                                                        dot_token_idx: TokenIdx(
                                                            559,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `right_mass`,
                                                            token_idx: TokenIdx(
                                                                560,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            561,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            23..23,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            562,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 23,
                                                        opr: AssignClosed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            552,
                                                        ),
                                                        ropd: 24,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            564,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
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
                                                    Expr::Binary {
                                                        lopd: 26,
                                                        opr: As,
                                                        opr_token_idx: TokenIdx(
                                                            565,
                                                        ),
                                                        ropd: 27,
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            3..9,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            566,
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
                                                data: [
                                                    Stmt::Break {
                                                        break_token: BreakToken {
                                                            token_idx: TokenIdx(
                                                                541,
                                                            ),
                                                        },
                                                    },
                                                    Stmt::IfElse {
                                                        if_branch: IfBranch {
                                                            if_token: IfToken {
                                                                token_idx: TokenIdx(
                                                                    533,
                                                                ),
                                                            },
                                                            condition: Ok(
                                                                11,
                                                            ),
                                                            eol_colon: Ok(
                                                                Colon(
                                                                    EolColonToken {
                                                                        token_idx: TokenIdx(
                                                                            540,
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
                                                    Stmt::Eval {
                                                        expr_idx: 25,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                514,
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
                                                                    517,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 0,
                                                    },
                                                    Stmt::Assert {
                                                        assert_token: AssertToken {
                                                            token_idx: TokenIdx(
                                                                519,
                                                            ),
                                                        },
                                                        condition: 3,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                523,
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
                                                                    526,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: 4,
                                                    },
                                                    Stmt::ForExt {
                                                        forext_token: ForextToken {
                                                            token_idx: TokenIdx(
                                                                528,
                                                            ),
                                                        },
                                                        expr: 7,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        532,
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
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                542,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                545,
                                                            ),
                                                            frame_var_expr_idx: 13,
                                                            frame_var_ident: `j`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        12,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        17,
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
                                                                        550,
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
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                563,
                                                            ),
                                                        },
                                                        result: 28,
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
                                                                            515,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `top_k_row_span_sum`,
                                                                token_idx: TokenIdx(
                                                                    516,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            524,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `i`,
                                                                token_idx: TokenIdx(
                                                                    525,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Move,
                                                        Move,
                                                    ],
                                                },
                                                pattern_infos: [
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
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `top_k_row_span_sum`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `i`,
                                                            1,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
                                                        Mut,
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
                                                                ident: `k`,
                                                            },
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                517,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        567,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `top_k_row_span_sum`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                526,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        567,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `i`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                551,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        563,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `j`,
                                                                expr_idx: 13,
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
                                                    kind: Condition,
                                                    expr_idx: 3,
                                                },
                                                ExprRoot {
                                                    kind: LetStmtInitialValue,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: EvalExpr,
                                                    expr_idx: 25,
                                                },
                                                ExprRoot {
                                                    kind: ReturnExpr,
                                                    expr_idx: 28,
                                                },
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr_idx: 29,
                                                },
                                            ],
                                        },
                                    },
                                    body: Some(
                                        29,
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