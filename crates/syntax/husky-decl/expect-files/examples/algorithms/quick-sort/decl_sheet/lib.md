Ok(
    DeclSheet {
        [salsa id]: 24,
        decls: [
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort`, `Fn`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Fugitive(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                implicit_parameters: [
                                    ImplicitParameterDecl {
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
                                explicit_parameters: [
                                    ExplicitParameterDecl::Regular {
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
                                return_ty: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
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
                                                    items: [],
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
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Ord`,
                                                            token_idx: TokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                        symbol_modifier_keyword_group: Some(
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
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 3,
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
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Fugitive(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                implicit_parameters: [
                                    ImplicitParameterDecl {
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
                                explicit_parameters: [
                                    ExplicitParameterDecl::Regular {
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
                                    ExplicitParameterDecl::Regular {
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
                                    ExplicitParameterDecl::Regular {
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
                                return_ty: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
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
                                                    items: [],
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
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Ord`,
                                                            token_idx: TokenIdx(
                                                                45,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `isize`,
                                                            token_idx: TokenIdx(
                                                                58,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `isize`,
                                                            token_idx: TokenIdx(
                                                                62,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                        symbol_modifier_keyword_group: Some(
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
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `low`,
                                                            token_idx: TokenIdx(
                                                                56,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
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
                                                    None,
                                                    None,
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
                                                    None,
                                                    None,
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
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            57,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            61,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 1,
                                                        ty_expr_idx: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 2,
                                                        ty_expr_idx: 5,
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
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Fugitive(
                        FugitivePath(`quick_sort::partition`, `Fn`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Fugitive(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`quick_sort::partition`, `Fn`),
                                implicit_parameters: [
                                    ImplicitParameterDecl {
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
                                explicit_parameters: [
                                    ExplicitParameterDecl::Regular {
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
                                    ExplicitParameterDecl::Regular {
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
                                    ExplicitParameterDecl::Regular {
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
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 6,
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
                                                            path: FugitivePath(`quick_sort::partition`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
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
                                                    items: [],
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
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Ord`,
                                                            token_idx: TokenIdx(
                                                                106,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `isize`,
                                                            token_idx: TokenIdx(
                                                                119,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `isize`,
                                                            token_idx: TokenIdx(
                                                                123,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `isize`,
                                                            token_idx: TokenIdx(
                                                                126,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                        symbol_modifier_keyword_group: Some(
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
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `low`,
                                                            token_idx: TokenIdx(
                                                                117,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
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
                                                    None,
                                                    None,
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
                                                    None,
                                                    None,
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
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            118,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            122,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 1,
                                                        ty_expr_idx: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 2,
                                                        ty_expr_idx: 5,
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
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)