Ok(
    SynDeclSheet {
        [salsa id]: 8,
        decls: [
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::logic::Prop`, `Extern`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemSynDecl::Type(
                        TypeSynDecl::Extern(
                            ExternTypeSynDecl {
                                path: TypePath(`core::logic::Prop`, `Extern`),
                                generic_parameters: [],
                                expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::logic::Prop`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
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
                                        symbol_region: SynSymbolRegion {
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
                                        roots: [],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::logic::LogicAnd`, `Structure`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemSynDecl::Type(
                        TypeSynDecl::Structure(
                            StructureTypeSynDecl {
                                path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                generic_parameters: [
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `P`,
                                                token_idx: TokenIdx(
                                                    8,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken(
                                                        TokenIdx(
                                                            9,
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Q`,
                                                token_idx: TokenIdx(
                                                    12,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken(
                                                        TokenIdx(
                                                            13,
                                                        ),
                                                    ),
                                                    1,
                                                ),
                                            ),
                                        },
                                    },
                                ],
                                expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::logic::Prop`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::logic::Prop`, `Extern`),
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
                                                            ident: `Prop`,
                                                            token_idx: TokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::logic::Prop`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Prop`,
                                                            token_idx: TokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::logic::Prop`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            9,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `P`,
                                                                    token_idx: TokenIdx(
                                                                        8,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            13,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Q`,
                                                                    token_idx: TokenIdx(
                                                                        12,
                                                                    ),
                                                                },
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
                                            SynExprRoot {
                                                kind: Traits,
                                                expr_idx: 0,
                                            },
                                            SynExprRoot {
                                                kind: Traits,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::logic::LogicOr`, `Inductive`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemSynDecl::Type(
                        TypeSynDecl::Inductive(
                            InductiveTypeSynDecl {
                                path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                generic_parameters: [
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `P`,
                                                token_idx: TokenIdx(
                                                    30,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken(
                                                        TokenIdx(
                                                            31,
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Q`,
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken(
                                                        TokenIdx(
                                                            35,
                                                        ),
                                                    ),
                                                    1,
                                                ),
                                            ),
                                        },
                                    },
                                ],
                                expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntitySynNodePath::ModuleItem(
                                                ModuleItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::logic::Prop`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::logic::Prop`, `Extern`),
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
                                                            ident: `Prop`,
                                                            token_idx: TokenIdx(
                                                                32,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::logic::Prop`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Prop`,
                                                            token_idx: TokenIdx(
                                                                36,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::logic::Prop`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternSynExprRegion {
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            31,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `P`,
                                                                    token_idx: TokenIdx(
                                                                        30,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            35,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Q`,
                                                                    token_idx: TokenIdx(
                                                                        34,
                                                                    ),
                                                                },
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
                                            SynExprRoot {
                                                kind: Traits,
                                                expr_idx: 0,
                                            },
                                            SynExprRoot {
                                                kind: Traits,
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
        ],
    },
)