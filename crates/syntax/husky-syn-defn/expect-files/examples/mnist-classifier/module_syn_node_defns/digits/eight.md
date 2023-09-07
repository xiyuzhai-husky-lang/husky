Ok(
    [
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Val(
                    ValSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ValSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 19,
                            colon_token: Ok(
                                Some(
                                    ColonToken(
                                        TokenIdx(
                                            7,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqToken(
                                    TokenIdx(
                                        9,
                                    ),
                                ),
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
                                                        path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                        ident: `FermiMatchResult`,
                                                        token_idx: TokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                            expr_idx: 1,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            6,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                ident: `FermiMatchResult`,
                                                                token_idx: TokenIdx(
                                                                    8,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                    expr_idx: 1,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                14,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                16,
                                            ),
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 1,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                11,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            13,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                17,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        10,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        12,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `big_mouth`,
                                                    token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Eval {
                                            expr_idx: 5,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                    ],
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
                                        kind: EvalExpr,
                                        expr_idx: 5,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Val(
                    ValSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ValSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 20,
                            colon_token: Ok(
                                Some(
                                    ColonToken(
                                        TokenIdx(
                                            24,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 5,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqToken(
                                    TokenIdx(
                                        30,
                                    ),
                                ),
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
                                                        path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Eight`,
                                                        },
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAll`,
                                                        token_idx: TokenIdx(
                                                            25,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            26,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subitem {
                                                parent: 3,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        28,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `Eight`,
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Eight`,
                                                        },
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
                                            expr_idx: 5,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            29,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 4,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `Eight`,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 3,
                                                        argument_expr_idx: 4,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `OneVsAll`,
                                                                token_idx: TokenIdx(
                                                                    25,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    26,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    27,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Subitem {
                                                        parent: 3,
                                                        scope_resolution_token: ScopeResolutionToken(
                                                            TokenIdx(
                                                                28,
                                                            ),
                                                        ),
                                                        ident_token: Ok(
                                                            IdentToken {
                                                                ident: `Eight`,
                                                                token_idx: TokenIdx(
                                                                    29,
                                                                ),
                                                            },
                                                        ),
                                                        path: Ok(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `Eight`,
                                                                },
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
                                                    expr_idx: 5,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 1,
                                            be_token_idx: TokenIdx(
                                                33,
                                            ),
                                            target: Ok(
                                                BeVariablesObelisk {
                                                    pattern_expr: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 3,
                                            be_token_idx: TokenIdx(
                                                37,
                                            ),
                                            target: Ok(
                                                BeVariablesObelisk {
                                                    pattern_expr: 2,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 5,
                                            be_token_idx: TokenIdx(
                                                41,
                                            ),
                                            target: Ok(
                                                BeVariablesObelisk {
                                                    pattern_expr: 3,
                                                    variables: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 7,
                                            be_token_idx: TokenIdx(
                                                45,
                                            ),
                                            target: Ok(
                                                BeVariablesObelisk {
                                                    pattern_expr: 4,
                                                    variables: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 6,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 9,
                                            dot_token_idx: TokenIdx(
                                                51,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    52,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                55,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    56,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 11,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                53,
                                            ),
                                            ropd: 12,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 7,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 14,
                                            dot_token_idx: TokenIdx(
                                                59,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    60,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 15,
                                            dot_token_idx: TokenIdx(
                                                61,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    62,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                64,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 16,
                                            lbox_token_idx: TokenIdx(
                                                63,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 17,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                65,
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 18,
                                            be_token_idx: TokenIdx(
                                                66,
                                            ),
                                            target: Ok(
                                                BeVariablesObelisk {
                                                    pattern_expr: 6,
                                                    variables: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 8,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 20,
                                            dot_token_idx: TokenIdx(
                                                71,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    72,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                73,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    74,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                76,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 22,
                                            lbox_token_idx: TokenIdx(
                                                75,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 23,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                77,
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 24,
                                            be_token_idx: TokenIdx(
                                                78,
                                            ),
                                            target: Ok(
                                                BeVariablesObelisk {
                                                    pattern_expr: 7,
                                                    variables: ArenaIdxRange(
                                                        7..8,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                82,
                                            ),
                                            Literal::Bool(
                                                False,
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                84,
                                            ),
                                            Literal::Bool(
                                                False,
                                            ),
                                        ),
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 10,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `Yes`,
                                                    },
                                                ),
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                4..11,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_one`,
                                                    token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_six`,
                                                    token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_zero`,
                                                    token_idx: TokenIdx(
                                                        40,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_seven`,
                                                    token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        70,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `OneVsAll`,
                                                    token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Subitem {
                                            parent: 9,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    86,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `Yes`,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    81,
                                                ),
                                            },
                                            condition: 26,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                condition: Ok(
                                                    25,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                80,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    83,
                                                ),
                                            },
                                            condition: 27,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                            },
                                            condition: 2,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                            condition: 4,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                            },
                                            condition: 6,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                            },
                                            condition: 8,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    47,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 5,
                                                    variables: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        49,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 13,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                },
                                                condition: Ok(
                                                    19,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                68,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        2..4,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 28,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        42,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `upper_excess`,
                                                    token_idx: TokenIdx(
                                                        48,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        67,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            SynPatternSymbol::Atom(
                                                1,
                                            ),
                                            SynPatternSymbol::Atom(
                                                2,
                                            ),
                                            SynPatternSymbol::Atom(
                                                3,
                                            ),
                                            SynPatternSymbol::Atom(
                                                4,
                                            ),
                                            SynPatternSymbol::Atom(
                                                5,
                                            ),
                                            SynPatternSymbol::Atom(
                                                6,
                                            ),
                                            SynPatternSymbol::Atom(
                                                7,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `none`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                3,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                4,
                                            ),
                                        ],
                                        [
                                            (
                                                `upper_excess`,
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                7,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                        ],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    35,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            88,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    39,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            88,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    43,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            88,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    47,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            88,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    49,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            88,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `upper_excess`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    68,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            85,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    80,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            83,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 2,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 4,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 6,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 8,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 13,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 26,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 27,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 28,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 29,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 21,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                RitchieParameters {
                                    lpar: LparToken(
                                        TokenIdx(
                                            90,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        SpecificParameterObelisk::Regular {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    92,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparToken(
                                        TokenIdx(
                                            95,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            96,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 4,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            99,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    93,
                                                ),
                                                opd: 1,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    97,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            94,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            98,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            91,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                None,
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `cc`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                None,
                                            ],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        92,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 1,
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            22,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            93,
                                                        ),
                                                        opd: 1,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                        opd: 3,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    94,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    98,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                    data: [
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    91,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        None,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        SynPatternSymbol::Atom(
                                                            1,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            1,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        None,
                                                    ],
                                                },
                                            },
                                            symbol_region: SynSymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                92,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 1,
                                                            ty_expr_idx: 2,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 2,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 4,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExpr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                101,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                102,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                104,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    105,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                106,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                107,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                109,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 108,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                108,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                112,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                113,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 7,
                                            dot_token_idx: TokenIdx(
                                                115,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `first`,
                                                token_idx: TokenIdx(
                                                    116,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                117,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                118,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 8,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                119,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 9,
                                            dot_token_idx: TokenIdx(
                                                120,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    121,
                                                ),
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                125,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                126,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    127,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 12,
                                            dot_token_idx: TokenIdx(
                                                128,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `first`,
                                                token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                130,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                131,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 13,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                132,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 14,
                                            dot_token_idx: TokenIdx(
                                                133,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    123,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 15,
                                            dot_token_idx: TokenIdx(
                                                135,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    136,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 16,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                124,
                                            ),
                                            ropd: 17,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                137,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 19,
                                            dot_token_idx: TokenIdx(
                                                138,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    139,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 20,
                                            dot_token_idx: TokenIdx(
                                                140,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                142,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                143,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                2..4,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    111,
                                                ),
                                            },
                                            condition: 18,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                },
                                                condition: Ok(
                                                    5,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                110,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 21,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                    ],
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
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
                                        kind: Condition,
                                        expr_idx: 18,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 21,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 22,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)