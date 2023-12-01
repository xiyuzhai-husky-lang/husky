SynDeclSheet {
    decls: [
        (
            ItemPath::MajorItem(
                MajorItemPath::Trait(
                    TraitPath(`core::clone::Clone`),
                ),
            ),
            SynDecl::MajorItem(
                MajorItemSynDecl::Trait(
                    TraitSynDecl {
                        path: TraitPath(`core::clone::Clone`),
                        template_parameters: [],
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TraitPath(`core::clone::Clone`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
                                symbol_region: SynSymbolRegionData {
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
                                syn_pattern_to_current_syn_symbol_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemPath::ImplBlock(
                ImplBlockPath::TraitForTypeImplBlock(
                    TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `core::clone`,
                            trai_path: TraitPath(`core::clone::Clone`),
                            ty_sketch: TypeSketch::DeriveAny,
                            disambiguator: 0,
                        },
                    },
                ),
            ),
            SynDecl::ImplBlock(
                ImplBlockSynDecl::TraitForType(
                    TraitForTypeImplBlockSynDecl {
                        path: TraitForTypeImplBlock {
                            data: TraitForTypeImplBlockPathData {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_sketch: TypeSketch::DeriveAny,
                                disambiguator: 0,
                            },
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
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePathData {
                                                                path: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `core::clone`,
                                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                                        ty_sketch: TypeSketch::DeriveAny,
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
                                symbol_region: SynSymbolRegionData {
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
                                syn_pattern_to_current_syn_symbol_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemPath::AssociatedItem(
                AssociatedItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `core::clone`,
                                                trai_path: TraitPath(`core::clone::Clone`),
                                                ty_sketch: TypeSketch::DeriveAny,
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `clone`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            SynDecl::AssociatedItem(
                AssociatedItemSynDecl::TraitForTypeItem(
                    TraitForTypeItemSynDecl::MethodFn(
                        TraitForTypeMethodFnSynDecl {
                            path: TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
                                            TraitForTypeItemPathData {
                                                impl_block: TraitForTypeImplBlock {
                                                    data: TraitForTypeImplBlockPathData {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_sketch: TypeSketch::DeriveAny,
                                                        disambiguator: 0,
                                                    },
                                                },
                                                ident: `clone`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlock {
                                                                                    data: TraitForTypeImplBlockPathData {
                                                                                        module_path: `core::clone`,
                                                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                                                        ty_sketch: TypeSketch::DeriveAny,
                                                                                        disambiguator: 0,
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
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
                                                symbol_region: SynSymbolRegionData {
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
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TraitForTypeItemPath(
                                                                            ItemPathId {
                                                                                data: ItemPathData::AssociatedItem(
                                                                                    AssociatedItemPathData::TraitForTypeItem(
                                                                                        TraitForTypeItemPathData {
                                                                                            impl_block: TraitForTypeImplBlock {
                                                                                                data: TraitForTypeImplBlockPathData {
                                                                                                    module_path: `core::clone`,
                                                                                                    trai_path: TraitPath(`core::clone::Clone`),
                                                                                                    ty_sketch: TypeSketch::DeriveAny,
                                                                                                    disambiguator: 0,
                                                                                                },
                                                                                            },
                                                                                            ident: `clone`,
                                                                                            item_kind: MethodFn,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
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
                                    symbol_region: SynSymbolRegionData {
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
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
    ],
}