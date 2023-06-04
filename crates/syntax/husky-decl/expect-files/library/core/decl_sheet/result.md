Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::result::Result`, `Enum`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::Enum(
                            EnumTypeDecl {
                                path: TypePath(`core::result::Result`, `Enum`),
                                ast_idx: 5,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
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
                                                                    ident: `T`,
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
                                                            7,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        6,
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
                                        roots: [],
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
                                                    traits: None,
                                                },
                                            },
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    traits: None,
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
                                                7,
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId::IllFormed(
                        IllFormedImplBlockId {
                            module: `core::result`,
                            disambiguator: 0,
                        },
                    ),
                ),
                Err(
                    DeclError::Derived(
                        DerivedDeclError::ImplErr,
                    ),
                ),
            ),
        ],
    },
)