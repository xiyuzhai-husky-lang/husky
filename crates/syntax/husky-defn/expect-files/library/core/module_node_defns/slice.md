Ok(
    [
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Type(
                TypeNodeDefn::Extern(
                    ExternTypeNodeDefn {
                        node_path: TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::slice::Slice`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: ExternTypeNodeDecl {
                            node_path: TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 0,
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                3,
                                            ),
                                        ),
                                        generic_parameters: [
                                            GenericParameterDecl {
                                                annotated_variance_token: Some(
                                                    VarianceToken::Covariant(
                                                        CovariantToken {
                                                            token_idx: TokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                symbol: 0,
                                                variant: GenericParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                6,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`core::slice::Slice`, `Extern`),
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
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
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
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)