Ok(
    [
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`std::ops::Add`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`std::ops::Add`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 3,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            8,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
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
                                    commas: [],
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
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    10,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                        attrs: TemplateParameterAttrs {
                                                            phantom: false,
                                                        },
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
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
                                            TemplateTypeParameter,
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