Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`std::ops::Add`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Trait(
                        TraitDefn {
                            path: TraitPath(`std::ops::Add`),
                            decl: TraitDecl {
                                path: TraitPath(`std::ops::Add`),
                                ast_idx: 3,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`std::ops::Add`),
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
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            10,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `B`,
                                                                    token_idx: TokenIdx(
                                                                        9,
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
                                implicit_parameter_decl_list: Ok(
                                    Some(
                                        ImplicitParameterDeclList {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    8,
                                                ),
                                            ),
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
                                                                            value: 105,
                                                                        },
                                                                    ),
                                                                ),
                                                                token_idx: TokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    traits: None,
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: Ok(
                                                RightAngleBracketToken(
                                                    TokenIdx(
                                                        10,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ],
    },
)