Ok(
    [
        ItemSynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`std::ops::Add`),
                    decl: TraitSynDecl {
                        path: TraitPath(`std::ops::Add`),
                        template_parameters: [
                            TemplateSynParameterData {
                                annotated_variance_token: None,
                                symbol: 1,
                                data: TemplateParameterSyndicateData::Type {
                                    ident_token: IdentRegionalToken {
                                        ident: `B`,
                                        regional_token_idx: RegionalTokenIdx(
                                            4,
                                        ),
                                    },
                                    traits: None,
                                },
                            },
                        ],
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
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
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [],
                                    },
                                    pattern_symbol_arena: Arena {
                                        data: [],
                                    },
                                    pattern_symbol_maps: [],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    5,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `B`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [],
                                syn_expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                syn_pattern_to_current_syn_symbol_map: [],
                            },
                        },
                    },
                },
            ),
        ),
    ],
)