Ok(
    SynNodeDeclSheet {
        [salsa id]: 3,
        decls: [
            (
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
                SynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Trait(
                        TraitSynNodeDecl {
                            syn_node_path: TraitSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`core::clone::Clone`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 3,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
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
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    ),
                ),
            ),
            (
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
                SynNodeDecl::ImplBlock(
                    ImplBlockSynNodeDecl::TraitForType(
                        TraitForTypeImplBlockSynNodeDecl {
                            syn_node_path: TraitForTypeImplBlockSynNodePath {
                                path: TraitForTypeImplBlockPath {
                                    module_path: `core::clone`,
                                    trai_path: TraitPath(`core::clone::Clone`),
                                    ty_sketch: TypeSketch::DeriveAny,
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 4,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            trai_expr: TraitObelisk {
                                expr: 0,
                            },
                            for_token: ConnectionForToken {
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                            self_ty_decl: DeriveAny {
                                at_token: AtToken(
                                    TokenIdx(
                                        18,
                                    ),
                                ),
                                derive_token: DeriveToken {
                                    token_idx: TokenIdx(
                                        19,
                                    ),
                                },
                                underscore_token: UnderscoreToken {
                                    token_idx: TokenIdx(
                                        20,
                                    ),
                                },
                            },
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            21,
                                        ),
                                    },
                                ),
                            ),
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
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 0,
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
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Clone`,
                                                        token_idx: TokenIdx(
                                                            16,
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
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Trait,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            (
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
                SynNodeDecl::AssociatedItem(
                    AssociatedItemSynNodeDecl::TraitForTypeItem(
                        TraitForTypeItemSynNodeDecl::MethodFn(
                            TraitForTypeMethodFnSynNodeDecl {
                                syn_node_path: TraitForTypeItemSynNodePath {
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
                                node: TraitForTypeItemSynNode {
                                    syn_node_path: TraitForTypeItemSynNodePath {
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
                                    ast_idx: 1,
                                    ident: `clone`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `core::clone`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 1,
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                parenate_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparToken(
                                            TokenIdx(
                                                24,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [],
                                        commas: [],
                                        rpar: RparToken(
                                            TokenIdx(
                                                25,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                26,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeBeforeColonObelisk {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Semicolon(
                                        EolSemicolonToken {
                                            token_idx: TokenIdx(
                                                28,
                                            ),
                                        },
                                    ),
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
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Clone`,
                                                                        token_idx: TokenIdx(
                                                                            16,
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
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: Trait,
                                                            expr_idx: 0,
                                                        },
                                                    ],
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
                                                SynExpr::SelfType(
                                                    TokenIdx(
                                                        27,
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
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 0,
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