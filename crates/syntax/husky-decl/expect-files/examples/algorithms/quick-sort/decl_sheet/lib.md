Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`quick_sort::quick_sort`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                ast_idx: 30,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`quick_sort::quick_sort`, `Fn`),
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
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        9,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `arr`,
                                                            token_idx: TokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
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
                                                        `arr`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
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
                                                        modifier: Const,
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
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: Traits,
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 3,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                3,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                    traits: Some(
                                                        (
                                                            ColonToken(
                                                                TokenIdx(
                                                                    5,
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                7,
                                            ),
                                        ),
                                    },
                                ),
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            8,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
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
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            16,
                                        ),
                                    ),
                                },
                                curry_token: None,
                                return_ty: None,
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
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
                            FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                ast_idx: 31,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
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
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        48,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `arr`,
                                                            token_idx: TokenIdx(
                                                                49,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `low`,
                                                            token_idx: TokenIdx(
                                                                56,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `high`,
                                                            token_idx: TokenIdx(
                                                                60,
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
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
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
                                                    PatternSymbol::Atom(
                                                        2,
                                                    ),
                                                ],
                                            },
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
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
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
                                                        modifier: Const,
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
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            50,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            57,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            61,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `high`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 1,
                                                        ty: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 2,
                                                        ty: 5,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: Traits,
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 3,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 4,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 5,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                42,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                    },
                                                    traits: Some(
                                                        (
                                                            ColonToken(
                                                                TokenIdx(
                                                                    44,
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                46,
                                            ),
                                        ),
                                    },
                                ),
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            47,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
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
                                        RegularParameterDeclPattern {
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
                                        RegularParameterDeclPattern {
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
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            63,
                                        ),
                                    ),
                                },
                                curry_token: None,
                                return_ty: None,
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            64,
                                        ),
                                    },
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
                            FugitivePath(`quick_sort::partition`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`quick_sort::partition`, `Fn`),
                                ast_idx: 32,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`quick_sort::partition`, `Fn`),
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
                                                        modifier_keyword_group: Some(
                                                            Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        109,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `arr`,
                                                            token_idx: TokenIdx(
                                                                110,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `low`,
                                                            token_idx: TokenIdx(
                                                                117,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `high`,
                                                            token_idx: TokenIdx(
                                                                121,
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
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
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
                                                    PatternSymbol::Atom(
                                                        2,
                                                    ),
                                                ],
                                            },
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
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
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
                                                        modifier: Const,
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
                                                        modifier: Mut,
                                                        access_start: TokenIdx(
                                                            111,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            118,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            122,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `high`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 1,
                                                        ty: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 2,
                                                        ty: 5,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: Traits,
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 3,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 4,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 5,
                                            },
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 6,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                103,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            104,
                                                        ),
                                                    },
                                                    traits: Some(
                                                        (
                                                            ColonToken(
                                                                TokenIdx(
                                                                    105,
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                107,
                                            ),
                                        ),
                                    },
                                ),
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            108,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
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
                                        RegularParameterDeclPattern {
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
                                        RegularParameterDeclPattern {
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
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            124,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            125,
                                        ),
                                    ),
                                ),
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 6,
                                    },
                                ),
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            127,
                                        ),
                                    },
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
                            FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                        ),
                    ),
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectedParameterDeclList(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        228,
                                    ),
                                    drained: false,
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                        ),
                    ),
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectedParameterDeclList(
                                TokenStreamState {
                                    next_token_idx: TokenIdx(
                                        288,
                                    ),
                                    drained: false,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)