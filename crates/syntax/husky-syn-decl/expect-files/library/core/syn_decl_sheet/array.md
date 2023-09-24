Ok(
    SynDeclSheet {
        [salsa id]: 1,
        decls: [
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::array::Array`, `Extern`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::Extern(
                            ExternTypeSynDecl {
                                path: TypePath(`core::array::Array`, `Extern`),
                                template_parameters: [
                                    TemplateParameterObelisk {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        data: TemplateParameterObeliskData::Constant {
                                            const_token: ConstRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                            ident_token: IdentRegionalToken {
                                                ident: `L`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                            colon_token: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                            ty_expr: 1,
                                        },
                                    },
                                    TemplateParameterObelisk {
                                        annotated_variance_token: Some(
                                            VarianceRegionalToken::Covariant(
                                                CovariantRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                            ),
                                        ),
                                        symbol: 2,
                                        data: TemplateParameterObeliskData::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
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
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::array::Array`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `usize`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `L`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 1,
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: Some(
                                                                VarianceRegionalToken::Covariant(
                                                                    CovariantRegionalToken {
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
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
                                                (
                                                    TemplateTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    TemplateTypeParameter,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ConstantImplicitParameterType,
                                                expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
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