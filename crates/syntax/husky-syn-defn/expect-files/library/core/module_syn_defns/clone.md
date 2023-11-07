Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::clone::Clone`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::clone::Clone`),
                        template_parameters: [],
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::clone::Clone`),
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
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                syn_pattern_expr_roots: [],
                                syn_expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                            },
                        },
                    },
                },
            ),
        ),
        SynDefn::ImplBlock(
            ImplBlockSynDecl::TraitForType(
                TraitForTypeImplBlockSynDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `core::clone`,
                        trai_path: TraitPath(`core::clone::Clone`),
                        ty_sketch: TypeSketch::DeriveAny,
                        disambiguator: 0,
                    },
                    template_parameters: [],
                    trai_expr: TraitSyndicate {
                        expr: 1,
                    },
                    self_ty_decl: DeriveAny {
                        pound_token: PoundRegionalToken(
                            RegionalTokenIdx(
                                4,
                            ),
                        ),
                        derive_token: DeriveRegionalToken {
                            token_idx: RegionalTokenIdx(
                                5,
                            ),
                        },
                        underscore_token: UnderscoreRegionalToken {
                            token_idx: RegionalTokenIdx(
                                6,
                            ),
                        },
                    },
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `core::clone`,
                                                trai_path: TraitPath(`core::clone::Clone`),
                                                ty_sketch: TypeSketch::DeriveAny,
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::clone::Clone`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `Clone`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::clone::Clone`),
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
                                inherited_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            syn_pattern_expr_roots: [],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::Trait,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                },
            ),
        ),
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TraitForTypeItem(
                TraitForTypeItemSynDefn::MethodFn(
                    TraitForTypeMethodFnSynDefn {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_sketch: TypeSketch::DeriveAny,
                                disambiguator: 0,
                            },
                            ident: `clone`,
                            item_kind: MethodFn,
                        },
                        decl: TraitForTypeMethodFnSynDecl {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::clone`,
                                    trai_path: TraitPath(`core::clone::Clone`),
                                    ty_sketch: TypeSketch::DeriveAny,
                                    disambiguator: 0,
                                },
                                ident: `clone`,
                                item_kind: MethodFn,
                            },
                            template_parameters: [],
                            self_value_parameter: None,
                            parenate_parameters: [],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `core::clone`,
                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                    ty_sketch: TypeSketch::DeriveAny,
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::clone::Clone`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Clone`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::clone::Clone`),
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
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitForTypeItemPath {
                                                            impl_block: TraitForTypeImplBlockPath {
                                                                module_path: `core::clone`,
                                                                trai_path: TraitPath(`core::clone::Clone`),
                                                                ty_sketch: TypeSketch::DeriveAny,
                                                                disambiguator: 0,
                                                            },
                                                            ident: `clone`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfType(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                        ],
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
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: None,
                    },
                ),
            ),
        ),
    ],
)