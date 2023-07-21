Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`std::ops::Add`),
                    decl: TraitDecl {
                        path: TraitPath(`std::ops::Add`),
                        ast_idx: 3,
                        generic_parameters: [
                            GenericParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: GenericParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `B`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                    traits: None,
                                },
                            },
                        ],
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`std::ops::Add`),
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
                                                    10,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
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
    ],
)