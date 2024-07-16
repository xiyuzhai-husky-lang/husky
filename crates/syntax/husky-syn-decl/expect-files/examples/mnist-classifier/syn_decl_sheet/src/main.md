```rust
SynDeclSheet {
    decls: [
        (
            ItemPath(`mnist_classifier::connected_component`),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(`mnist_classifier::connected_component),
                },
            ),
        ),
        (
            ItemPath(`mnist_classifier::raw_contour`),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(`mnist_classifier::raw_contour),
                },
            ),
        ),
        (
            ItemPath(`mnist_classifier::geom2d`),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(`mnist_classifier::geom2d),
                },
            ),
        ),
        (
            ItemPath(`mnist_classifier::line_segment_sketch`),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch),
                },
            ),
        ),
        (
            ItemPath(`mnist_classifier::fermi`),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(`mnist_classifier::fermi),
                },
            ),
        ),
        (
            ItemPath(`mnist_classifier::digits`),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(`mnist_classifier::digits),
                },
            ),
        ),
        (
            ItemPath(`mnist_classifier::major`),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(`mnist_classifier::major),
                },
            ),
        ),
        (
            ItemPath(`mnist_classifier::main`),
            SynDecl::MajorItem(
                MajorItemSynDecl::Form(
                    FormSynDecl::Val(
                        MajorValSynDecl {
                            path: MajorFormPath(`mnist_classifier::main`, `Val`),
                            return_ty: ReturnTypeBeforeEqSyndicate {
                                expr: 2,
                            },
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::main`, `Val`, (0)),
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
                                                            TypePath(`malamute::Class`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
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