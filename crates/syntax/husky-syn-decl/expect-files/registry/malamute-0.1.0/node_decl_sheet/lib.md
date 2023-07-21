Ok(
    NodeDeclSheet {
        [salsa id]: 48,
        decls: [
            (
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeNodeDecl::Enum(
                            EnumTypeNodeDecl {
                                node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 6,
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    3,
                                                ),
                                            ),
                                            generic_parameters: [
                                                GenericParameterDecl {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: GenericParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                                GenericParameterDecl {
                                                    annotated_variance_token: None,
                                                    symbol: 1,
                                                    variant: GenericParameterDeclPatternVariant::Constant {
                                                        const_token: ConstToken {
                                                            token_idx: TokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                        colon_token: ColonToken(
                                                            TokenIdx(
                                                                8,
                                                            ),
                                                        ),
                                                        ty_expr: 0,
                                                    },
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        5,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    10,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                expr_region: SynExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        9,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [],
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
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            10,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        7,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 0,
                                                            },
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ConstantImplicitParameterType,
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
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeNodeDecl::Enum(
                            EnumTypeNodeDecl {
                                node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 7,
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    18,
                                                ),
                                            ),
                                            generic_parameters: [
                                                GenericParameterDecl {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: GenericParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                19,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                                GenericParameterDecl {
                                                    annotated_variance_token: None,
                                                    symbol: 1,
                                                    variant: GenericParameterDeclPatternVariant::Constant {
                                                        const_token: ConstToken {
                                                            token_idx: TokenIdx(
                                                                21,
                                                            ),
                                                        },
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                22,
                                                            ),
                                                        },
                                                        colon_token: ColonToken(
                                                            TokenIdx(
                                                                23,
                                                            ),
                                                        ),
                                                        ty_expr: 0,
                                                    },
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        20,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    25,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                expr_region: SynExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        24,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    19,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [],
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
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            20,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            25,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        22,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 0,
                                                            },
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ConstantImplicitParameterType,
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
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Fugitive(
                        FugitiveNodeDecl::Gn(
                            GnNodeDecl {
                                node_path: FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 9,
                                expr_region: SynExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    63,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    TokenIdx(
                                                        82,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::PrincipalEntityPath {
                                                    entity_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    63,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 4,
                                                    argument_expr_idx: 5,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `label`,
                                                    token_idx: TokenIdx(
                                                        88,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    66,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 6,
                                                    argument_expr_idx: 7,
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                76,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                80,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `OneVsAllResult`,
                                                            token_idx: TokenIdx(
                                                                86,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `skip`,
                                                            token_idx: TokenIdx(
                                                                78,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    None,
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
                                                        `skip`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
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
                                                            64,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        63,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            69,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        66,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 0,
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            72,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitVariadicParameter {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `f`,
                                                                token_idx: TokenIdx(
                                                                    74,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            79,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                            ident: `skip`,
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
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitVariadicParameter {
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ConstantImplicitParameterType,
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 2,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterDefaultValue {
                                                    ty_expr_idx: 2,
                                                },
                                                expr_idx: 3,
                                            },
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 8,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    62,
                                                ),
                                            ),
                                            generic_parameters: [
                                                GenericParameterDecl {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: GenericParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                63,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                                GenericParameterDecl {
                                                    annotated_variance_token: None,
                                                    symbol: 1,
                                                    variant: GenericParameterDeclPatternVariant::Constant {
                                                        const_token: ConstToken {
                                                            token_idx: TokenIdx(
                                                                65,
                                                            ),
                                                        },
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                66,
                                                            ),
                                                        },
                                                        colon_token: ColonToken(
                                                            TokenIdx(
                                                                67,
                                                            ),
                                                        ),
                                                        ty_expr: 0,
                                                    },
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        64,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    69,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                parenic_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                70,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenic_parameters: [
                                            SpecificParameterDecl::Variadic {
                                                dot_dot_dot_token: DotDotDotToken(
                                                    TokenIdx(
                                                        71,
                                                    ),
                                                ),
                                                variadic_variant: VariadicVariant::Vec {
                                                    lbox_token: LeftBoxBracketToken(
                                                        TokenIdx(
                                                            72,
                                                        ),
                                                    ),
                                                    rbox_token: RightBoxBracketToken(
                                                        TokenIdx(
                                                            73,
                                                        ),
                                                    ),
                                                },
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `f`,
                                                    token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                },
                                                variable: 2,
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        75,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                            SpecificParameterDecl::Keyed {
                                                pattern: 0,
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `skip`,
                                                    token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                },
                                                variable: 3,
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        79,
                                                    ),
                                                ),
                                                ty: 2,
                                                eq_token: EqToken(
                                                    TokenIdx(
                                                        81,
                                                    ),
                                                ),
                                                default: Right(
                                                    3,
                                                ),
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    77,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    83,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                84,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                85,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeExprBeforeColon {
                                            expr: 8,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Semicolon(
                                        EolSemicolonToken {
                                            token_idx: TokenIdx(
                                                89,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                EntitySynNodePath::ImplBlock(
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: Path(
                                    TypePath(
                                        Id {
                                            value: 104,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ImplBlock(
                    ImplBlockSynNodeDecl::TraitForType(
                        TraitForTypeImplBlockNodeDecl {
                            node_path: TraitForTypeImplBlockSynNodePath {
                                path: TraitForTypeImplBlockPath {
                                    module_path: `malamute`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 104,
                                            },
                                        ),
                                    ),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 8,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                33,
                                            ),
                                        ),
                                        generic_parameters: [
                                            GenericParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: GenericParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            GenericParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: GenericParameterDeclPatternVariant::Constant {
                                                    const_token: ConstToken {
                                                        token_idx: TokenIdx(
                                                            36,
                                                        ),
                                                    },
                                                    ident_token: IdentToken {
                                                        ident: `label`,
                                                        token_idx: TokenIdx(
                                                            37,
                                                        ),
                                                    },
                                                    colon_token: ColonToken(
                                                        TokenIdx(
                                                            38,
                                                        ),
                                                    ),
                                                    ty_expr: 0,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    35,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                40,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            trai_expr: TraitExpr {
                                expr: 7,
                            },
                            for_token: ConnectionForToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeExpr {
                                    expr: 12,
                                },
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            53,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: SynExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: Path(
                                                            TypePath(
                                                                Id {
                                                                    value: 104,
                                                                },
                                                            ),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::ops::Unveil`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    47,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    48,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                37,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    51,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 8,
                                                argument_expr_idx: 9,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    52,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                37,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 10,
                                                argument_expr_idx: 11,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `core`,
                                                        token_idx: TokenIdx(
                                                            41,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core`,
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subentity {
                                                parent: 0,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        42,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `ops`,
                                                        token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        `core::ops`,
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subentity {
                                                parent: 1,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        44,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `Unveil`,
                                                        token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::ops::Unveil`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAllResult`,
                                                        token_idx: TokenIdx(
                                                            46,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAll`,
                                                        token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            data: [
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        35,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    34,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        40,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    37,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                        ExprRoot {
                                            kind: Trait,
                                            expr_idx: 7,
                                        },
                                        ExprRoot {
                                            kind: SelfType,
                                            expr_idx: 12,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                EntitySynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TraitForTypeItem(
                        TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: Path(
                                            TypePath(
                                                Id {
                                                    value: 104,
                                                },
                                            ),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Output`,
                                    item_kind: AssociatedType,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::AssociatedItem(
                    AssociatedItemSynNodeDecl::TraitForTypeItem(
                        TraitForTypeItemNodeDecl::AssociatedType(
                            TraitForTypeAssociatedTypeNodeDecl {
                                node_path: TraitForTypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: Path(
                                                    TypePath(
                                                        Id {
                                                            value: 104,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `Output`,
                                            item_kind: AssociatedType,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                node: TraitForTypeItemNode {
                                    node_path: TraitForTypeItemSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitForTypeItemPath {
                                                impl_block: TraitForTypeImplBlockPath {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: Path(
                                                        TypePath(
                                                            Id {
                                                                value: 104,
                                                            },
                                                        ),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                                ident: `Output`,
                                                item_kind: AssociatedType,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 5,
                                    ident: `Output`,
                                    item_kind: AssociatedType,
                                    visibility: Scope::PubUnder(
                                        `malamute`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 5,
                                generics: Ok(
                                    None,
                                ),
                                eq_token: Ok(
                                    EqToken(
                                        TokenIdx(
                                            56,
                                        ),
                                    ),
                                ),
                                ty_term_expr_idx: 0,
                                expr_region: SynExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntitySynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: Path(
                                                                            TypePath(
                                                                                Id {
                                                                                    value: 104,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    39,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                34,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                entity_path_expr: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::ops::Unveil`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                entity_path_expr: 3,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 1,
                                                                argument_expr_idx: 2,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    47,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                34,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 3,
                                                                argument_expr_idx: 4,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    48,
                                                                ),
                                                                current_symbol_idx: 1,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                                        ident_token: IdentToken {
                                                                            ident: `label`,
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 5,
                                                                argument_expr_idx: 6,
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                entity_path_expr: 4,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    51,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                34,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 8,
                                                                argument_expr_idx: 9,
                                                            },
                                                            SynExpr::CurrentSymbol {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    52,
                                                                ),
                                                                current_symbol_idx: 1,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                                        ident_token: IdentToken {
                                                                            ident: `label`,
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 10,
                                                                argument_expr_idx: 11,
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `core`,
                                                                        token_idx: TokenIdx(
                                                                            41,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::Module(
                                                                    `core`,
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Subentity {
                                                                parent: 0,
                                                                scope_resolution_token: ScopeResolutionToken(
                                                                    TokenIdx(
                                                                        42,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentToken {
                                                                        ident: `ops`,
                                                                        token_idx: TokenIdx(
                                                                            43,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::Module(
                                                                        `core::ops`,
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Subentity {
                                                                parent: 1,
                                                                scope_resolution_token: ScopeResolutionToken(
                                                                    TokenIdx(
                                                                        44,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentToken {
                                                                        ident: `Unveil`,
                                                                        token_idx: TokenIdx(
                                                                            45,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::ops::Unveil`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `OneVsAllResult`,
                                                                        token_idx: TokenIdx(
                                                                            46,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `OneVsAll`,
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                            data: [
                                                                CurrentSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        35,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `Label`,
                                                                                token_idx: TokenIdx(
                                                                                    34,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    modifier: Const,
                                                                    access_start: TokenIdx(
                                                                        40,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                            ident_token: IdentToken {
                                                                                ident: `label`,
                                                                                token_idx: TokenIdx(
                                                                                    37,
                                                                                ),
                                                                            },
                                                                            ty_expr_idx: 0,
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            (
                                                                ImplicitTypeParameter,
                                                                ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            ),
                                                            (
                                                                ImplicitTypeParameter,
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ConstantImplicitParameterType,
                                                            expr_idx: 0,
                                                        },
                                                        ExprRoot {
                                                            kind: Trait,
                                                            expr_idx: 7,
                                                        },
                                                        ExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 12,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: Path(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 104,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `Output`,
                                                                item_kind: AssociatedType,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::Unit {
                                                    lpar_token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [],
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
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `Label`,
                                                            },
                                                        ),
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Constant {
                                                                ident: `label`,
                                                            },
                                                        ),
                                                    },
                                                ],
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
                                                kind: AssociatedTypeTerm,
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
        ],
    },
)