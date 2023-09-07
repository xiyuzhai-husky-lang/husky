Ok(
    SynNodeDeclSheet {
        [salsa id]: 1,
        decls: [
            (
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
                ItemSynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Extern(
                            ExternTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::array::Array`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 2,
                                template_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LaOrLtToken(
                                                TokenIdx(
                                                    8,
                                                ),
                                            ),
                                            template_parameters: [
                                                TemplateParameterObelisk {
                                                    annotated_variance_token: None,
                                                    symbol: 1,
                                                    variant: TemplateParameterDeclPatternVariant::Constant {
                                                        const_token: ConstToken {
                                                            token_idx: TokenIdx(
                                                                9,
                                                            ),
                                                        },
                                                        ident_token: IdentToken {
                                                            ident: `L`,
                                                            token_idx: TokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                        colon_token: ColonToken(
                                                            TokenIdx(
                                                                11,
                                                            ),
                                                        ),
                                                        ty_expr: 1,
                                                    },
                                                },
                                                TemplateParameterObelisk {
                                                    annotated_variance_token: Some(
                                                        VarianceToken::Covariant(
                                                            CovariantToken {
                                                                token_idx: TokenIdx(
                                                                    14,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    symbol: 2,
                                                    variant: TemplateParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                15,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        13,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RaOrGtToken(
                                                TokenIdx(
                                                    16,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
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
                                                    item_path_expr: 1,
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
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `usize`,
                                                            token_idx: TokenIdx(
                                                                12,
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
                                                            13,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `L`,
                                                                    token_idx: TokenIdx(
                                                                        10,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 1,
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            16,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: Some(
                                                                VarianceToken::Covariant(
                                                                    CovariantToken {
                                                                        token_idx: TokenIdx(
                                                                            14,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        15,
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