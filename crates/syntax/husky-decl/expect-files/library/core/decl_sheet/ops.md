Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::ops::Add`),
                        ast_idx: 3,
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    DeclExprPath::Entity(
                                        TraitPath(`core::ops::Add`),
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
                                        data: [
                                            CurrentSymbol {
                                                ident: `B`,
                                                access_start: TokenIdx(
                                                    12,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                        ident_token: IdentifierToken {
                                                            ident: `B`,
                                                            token_idx: TokenIdx(
                                                                11,
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
                                        ImplicitTypeParameter,
                                    ],
                                },
                                roots: [],
                            },
                        },
                        implicit_parameter_decl_list: Some(
                            ImplicitParameterDeclList {
                                langle: LeftAngleBracketOrLessThanToken {
                                    token_idx: TokenIdx(
                                        10,
                                    ),
                                },
                                implicit_parameters: [
                                    ImplicitParameterDecl {
                                        pattern: ImplicitParameterDeclPattern {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: Type0 {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 49,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                },
                                            },
                                        },
                                        traits: None,
                                    },
                                ],
                                commas: [],
                                rangle: RightAngleBracketToken {
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                            },
                        ),
                    },
                ),
            ),
        ],
    },
)