Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::list::List`, `Alien`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::Alien(
                            AlienTypeDecl {
                                path: TypePath(`core::list::List`, `Alien`),
                                ast_idx: 0,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::list::List`, `Alien`),
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
                                                            6,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        5,
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
                                                    3,
                                                ),
                                            ),
                                            implicit_parameters: [
                                                ImplicitParameterDecl {
                                                    pattern: ImplicitParameterDeclPattern {
                                                        annotated_variance_token: Some(
                                                            Covariant(
                                                                CovariantToken {
                                                                    token_idx: TokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        symbol: 0,
                                                        variant: Type0 {
                                                            ident_token: IdentifierToken {
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 29,
                                                                        },
                                                                    ),
                                                                ),
                                                                token_idx: TokenIdx(
                                                                    5,
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
                                                        6,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)