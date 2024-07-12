```rust
SynNodeDeclSheet {
    decls: [
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist::Task`, `TypeVar`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::TypeVar(
                        TypeVarSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist::Task`, `TypeVar`, (0)),
                            eq_token: Ok(
                                Some(
                                    EqRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            default: Some(
                                0,
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist::Task`, `TypeVar`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::task::MnistTask`, `Extern`),
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
                                                        ident: `MnistTask`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::task::MnistTask`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::TypeVarDefault,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist::task`, `StaticVar`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::StaticVar(
                        MajorStaticVarSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist::task`, `StaticVar`, (0)),
                            colon_token: Ok(
                                ColonRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 0,
                                },
                            ),
                            opt_eq_token: Ok(
                                Some(
                                    EqRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                ),
                            ),
                            expr: Some(
                                2,
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist::task`, `StaticVar`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::task::MnistTask`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::MajorItemPathAssocItem {
                                                parent_expr_idx: 3,
                                                parent_path: MajorItemPath::Type(
                                                    TypePath(`mnist::task::MnistTask`, `Extern`),
                                                ),
                                                colon_colon_regional_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `new`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 1,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `mnist`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    ModulePath(`mnist`),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 0,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        7,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `task`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        ModulePath(`mnist::task`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `MnistTask`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::task::MnistTask`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `MnistTask`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::task::MnistTask`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::StaticExpr,
                                            syn_expr_idx: 2,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
    ],
}
```