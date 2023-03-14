Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Function(
                            FunctionDecl {
                                path: FormPath(`quick_sort::quick_sort`, `Function`),
                                ast_idx: 30,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort`, `Function`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BoxColonList {
                                                    lbox_token_idx: TokenIdx(
                                                        12,
                                                    ),
                                                    colon_token_idx: TokenIdx(
                                                        13,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `T`,
                                                    token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
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
                                                        6,
                                                    ),
                                                    ident: `Ord`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
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
                                                        ident_token: IdentToken {
                                                            ident: `arr`,
                                                            token_idx: TokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `arr`,
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
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                ImplicitTypeParameter,
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 3,
                                                },
                                            ],
                                        },
                                        roots: [],
                                    },
                                },
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        ImplicitParameterDeclList {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    3,
                                                ),
                                            ),
                                            implicit_parameters: [
                                                ImplicitParameterDecl {
                                                    pattern: ImplicitParameterDeclPattern {
                                                        annotated_variance_token: None,
                                                        symbol: 0,
                                                        variant: Type0 {
                                                            ident_token: IdentToken {
                                                                ident: Ident(
                                                                    Word(
                                                                        Id {
                                                                            value: 97,
                                                                        },
                                                                    ),
                                                                ),
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    traits: Some(
                                                        (
                                                            ColonToken(
                                                                TokenIdx(
                                                                    5,
                                                                ),
                                                            ),
                                                            Some(
                                                                0,
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: Ok(
                                                RightAngleBracketToken(
                                                    TokenIdx(
                                                        7,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                parameter_decl_list: Ok(
                                    ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                8,
                                            ),
                                        ),
                                        parameters: [
                                            ExplicitParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rpar: Ok(
                                            RightParenthesisToken(
                                                TokenIdx(
                                                    16,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectCurry(
                                            TokenIdx(
                                                17,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Err(
                                    Original(
                                        ExpectOutputType(
                                            TokenIdx(
                                                17,
                                            ),
                                        ),
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            17,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort_aux`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Function(
                            FunctionDecl {
                                path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                ast_idx: 31,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort_aux`, `Function`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BoxColonList {
                                                    lbox_token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                    colon_token_idx: TokenIdx(
                                                        52,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `T`,
                                                    token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    43,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 1,
                                                    argument: 2,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                        45,
                                                    ),
                                                    ident: `Ord`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                        ident_token: IdentToken {
                                                            ident: `arr`,
                                                            token_idx: TokenIdx(
                                                                49,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Ident {
                                                        ident_token: IdentToken {
                                                            ident: `low`,
                                                            token_idx: TokenIdx(
                                                                56,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Ident {
                                                        ident_token: IdentToken {
                                                            ident: `high`,
                                                            token_idx: TokenIdx(
                                                                60,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                                Parameter,
                                                Parameter,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `arr`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `low`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `high`,
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
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            44,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            50,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            57,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            61,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `high`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                ImplicitTypeParameter,
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 3,
                                                },
                                                RegularParameter {
                                                    pattern: 1,
                                                    ty: 4,
                                                },
                                                RegularParameter {
                                                    pattern: 2,
                                                    ty: 5,
                                                },
                                            ],
                                        },
                                        roots: [],
                                    },
                                },
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        ImplicitParameterDeclList {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    42,
                                                ),
                                            ),
                                            implicit_parameters: [
                                                ImplicitParameterDecl {
                                                    pattern: ImplicitParameterDeclPattern {
                                                        annotated_variance_token: None,
                                                        symbol: 0,
                                                        variant: Type0 {
                                                            ident_token: IdentToken {
                                                                ident: Ident(
                                                                    Word(
                                                                        Id {
                                                                            value: 97,
                                                                        },
                                                                    ),
                                                                ),
                                                                token_idx: TokenIdx(
                                                                    43,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    traits: Some(
                                                        (
                                                            ColonToken(
                                                                TokenIdx(
                                                                    44,
                                                                ),
                                                            ),
                                                            Some(
                                                                0,
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: Ok(
                                                RightAngleBracketToken(
                                                    TokenIdx(
                                                        46,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                parameter_decl_list: Ok(
                                    ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                47,
                                            ),
                                        ),
                                        parameters: [
                                            ExplicitParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        50,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                            ExplicitParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        57,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                            ExplicitParameterDeclPattern {
                                                pattern: 2,
                                                variables: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        61,
                                                    ),
                                                ),
                                                ty: 5,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    55,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    59,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rpar: Ok(
                                            RightParenthesisToken(
                                                TokenIdx(
                                                    63,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectCurry(
                                            TokenIdx(
                                                64,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Err(
                                    Original(
                                        ExpectOutputType(
                                            TokenIdx(
                                                64,
                                            ),
                                        ),
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            64,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::partition`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Function(
                            FunctionDecl {
                                path: FormPath(`quick_sort::partition`, `Function`),
                                ast_idx: 32,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::partition`, `Function`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BoxColonList {
                                                    lbox_token_idx: TokenIdx(
                                                        112,
                                                    ),
                                                    colon_token_idx: TokenIdx(
                                                        113,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `T`,
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    104,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 1,
                                                    argument: 2,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                        106,
                                                    ),
                                                    ident: `Ord`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        119,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                        ident_token: IdentToken {
                                                            ident: `arr`,
                                                            token_idx: TokenIdx(
                                                                110,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Ident {
                                                        ident_token: IdentToken {
                                                            ident: `low`,
                                                            token_idx: TokenIdx(
                                                                117,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Ident {
                                                        ident_token: IdentToken {
                                                            ident: `high`,
                                                            token_idx: TokenIdx(
                                                                121,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                                Parameter,
                                                Parameter,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `arr`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `low`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `high`,
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
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            105,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        104,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            111,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            118,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            122,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            ident: `high`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                ImplicitTypeParameter,
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 3,
                                                },
                                                RegularParameter {
                                                    pattern: 1,
                                                    ty: 4,
                                                },
                                                RegularParameter {
                                                    pattern: 2,
                                                    ty: 5,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr: 6,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        ImplicitParameterDeclList {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    103,
                                                ),
                                            ),
                                            implicit_parameters: [
                                                ImplicitParameterDecl {
                                                    pattern: ImplicitParameterDeclPattern {
                                                        annotated_variance_token: None,
                                                        symbol: 0,
                                                        variant: Type0 {
                                                            ident_token: IdentToken {
                                                                ident: Ident(
                                                                    Word(
                                                                        Id {
                                                                            value: 97,
                                                                        },
                                                                    ),
                                                                ),
                                                                token_idx: TokenIdx(
                                                                    104,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    traits: Some(
                                                        (
                                                            ColonToken(
                                                                TokenIdx(
                                                                    105,
                                                                ),
                                                            ),
                                                            Some(
                                                                0,
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: Ok(
                                                RightAngleBracketToken(
                                                    TokenIdx(
                                                        107,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                parameter_decl_list: Ok(
                                    ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                108,
                                            ),
                                        ),
                                        parameters: [
                                            ExplicitParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        111,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                            ExplicitParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        118,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                            ExplicitParameterDeclPattern {
                                                pattern: 2,
                                                variables: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        122,
                                                    ),
                                                ),
                                                ty: 5,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    116,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    120,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rpar: Ok(
                                            RightParenthesisToken(
                                                TokenIdx(
                                                    124,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken(
                                        TokenIdx(
                                            125,
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 6,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            127,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Feature(
                            FeatureDecl {
                                path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                                ast_idx: 34,
                                curry_token: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectCurry(
                                            TokenIdx(
                                                228,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Err(
                                    Original(
                                        ExpectOutputType(
                                            TokenIdx(
                                                228,
                                            ),
                                        ),
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            228,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [],
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
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Feature(
                            FeatureDecl {
                                path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                                ast_idx: 36,
                                curry_token: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectCurry(
                                            TokenIdx(
                                                288,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Err(
                                    Original(
                                        ExpectOutputType(
                                            TokenIdx(
                                                288,
                                            ),
                                        ),
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            288,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [],
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
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)