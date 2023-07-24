Ok(
    SynNodeDeclSheet {
        [salsa id]: 8,
        decls: [
            (
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
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Extern(
                            ExternTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::logic::Prop`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 2,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                syn_expr_region: SynExprRegion {
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
                                        principal_item_path_expr_arena: Arena {
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
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Structure(
                            StructureTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 3,
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    7,
                                                ),
                                            ),
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
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        11,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    15,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                syn_expr_region: SynExprRegion {
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
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::logic::Prop`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 1,
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
                                        principal_item_path_expr_arena: Arena {
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
                                                        variant: CurrentSynSymbolVariant::ImplicitParameter {
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
                                                        variant: CurrentSynSymbolVariant::ImplicitParameter {
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
                SynNodeDecl::ModuleItem(
                    ModuleItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Inductive(
                            InductiveTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 4,
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    29,
                                                ),
                                            ),
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
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        33,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    37,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                syn_expr_region: SynExprRegion {
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
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::logic::Prop`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 1,
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
                                        principal_item_path_expr_arena: Arena {
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
                                                        variant: CurrentSynSymbolVariant::ImplicitParameter {
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
                                                        variant: CurrentSynSymbolVariant::ImplicitParameter {
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