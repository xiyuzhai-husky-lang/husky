Ok(
    SynDeclSheet {
        [salsa id]: 25,
        decls: [
            (
                ItemPath::Submodule(
                    SubmodulePath(
                        `mnist_classifier::connected_component`,
                    ),
                ),
                SynDecl::Submodule(
                    SubmoduleSynDecl {
                        path: SubmodulePath(
                            `mnist_classifier::connected_component`,
                        ),
                        ast_idx: 11,
                    },
                ),
            ),
            (
                ItemPath::Submodule(
                    SubmodulePath(
                        `mnist_classifier::raw_contour`,
                    ),
                ),
                SynDecl::Submodule(
                    SubmoduleSynDecl {
                        path: SubmodulePath(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 12,
                    },
                ),
            ),
            (
                ItemPath::Submodule(
                    SubmodulePath(
                        `mnist_classifier::geom2d`,
                    ),
                ),
                SynDecl::Submodule(
                    SubmoduleSynDecl {
                        path: SubmodulePath(
                            `mnist_classifier::geom2d`,
                        ),
                        ast_idx: 13,
                    },
                ),
            ),
            (
                ItemPath::Submodule(
                    SubmodulePath(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                ),
                SynDecl::Submodule(
                    SubmoduleSynDecl {
                        path: SubmodulePath(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        ast_idx: 14,
                    },
                ),
            ),
            (
                ItemPath::Submodule(
                    SubmodulePath(
                        `mnist_classifier::fermi`,
                    ),
                ),
                SynDecl::Submodule(
                    SubmoduleSynDecl {
                        path: SubmodulePath(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 15,
                    },
                ),
            ),
            (
                ItemPath::Submodule(
                    SubmodulePath(
                        `mnist_classifier::digits`,
                    ),
                ),
                SynDecl::Submodule(
                    SubmoduleSynDecl {
                        path: SubmodulePath(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 16,
                    },
                ),
            ),
            (
                ItemPath::Submodule(
                    SubmodulePath(
                        `mnist_classifier::major`,
                    ),
                ),
                SynDecl::Submodule(
                    SubmoduleSynDecl {
                        path: SubmodulePath(
                            `mnist_classifier::major`,
                        ),
                        ast_idx: 17,
                    },
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::main`, `Val`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::Val(
                            ValSynDecl {
                                path: FugitivePath(`mnist_classifier::main`, `Val`),
                                return_ty: Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 2,
                                    },
                                ),
                                expr: None,
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::main`, `Val`),
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
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::Class`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 0,
                                                    argument_expr_idx: 1,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Class`,
                                                            token_idx: TokenIdx(
                                                                68,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::Class`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `MnistLabel`,
                                                            token_idx: TokenIdx(
                                                                69,
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
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 2,
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