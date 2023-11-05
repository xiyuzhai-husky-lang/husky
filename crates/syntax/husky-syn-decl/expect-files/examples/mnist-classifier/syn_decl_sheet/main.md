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
                            ValFugitiveSynDecl {
                                path: FugitivePath(`mnist_classifier::main`, `Val`),
                                return_ty: Some(
                                    ReturnTypeBeforeEqSyndicate {
                                        expr: 3,
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
                                                syn_expr_idx: 3,
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