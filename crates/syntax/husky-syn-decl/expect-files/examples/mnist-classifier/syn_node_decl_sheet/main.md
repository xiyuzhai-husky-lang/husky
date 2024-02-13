SynNodeDeclSheet {
    decls: [
        (
            ItemSynNodePath::Submodule(
                Room32,
                SubmoduleSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 1,
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::Submodule(
                SubmoduleSynNodeDecl {
                    syn_node_path: SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ItemSynNodePath::Submodule(
                Room32,
                SubmoduleSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 2,
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::Submodule(
                SubmoduleSynNodeDecl {
                    syn_node_path: SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ItemSynNodePath::Submodule(
                Room32,
                SubmoduleSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 3,
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::Submodule(
                SubmoduleSynNodeDecl {
                    syn_node_path: SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ItemSynNodePath::Submodule(
                Room32,
                SubmoduleSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 4,
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::Submodule(
                SubmoduleSynNodeDecl {
                    syn_node_path: SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ItemSynNodePath::Submodule(
                Room32,
                SubmoduleSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 5,
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::Submodule(
                SubmoduleSynNodeDecl {
                    syn_node_path: SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ItemSynNodePath::Submodule(
                Room32,
                SubmoduleSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 6,
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::Submodule(
                SubmoduleSynNodeDecl {
                    syn_node_path: SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ItemSynNodePath::Submodule(
                Room32,
                SubmoduleSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 7,
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::Submodule(
                SubmoduleSynNodeDecl {
                    syn_node_path: SubmoduleSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::Val(
                        MajorValSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 8,
                                    },
                                ),
                            ),
                            colon_token: Ok(
                                ColonRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 3,
                                },
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::Class`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Class`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::Class`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `MnistLabel`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 3,
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