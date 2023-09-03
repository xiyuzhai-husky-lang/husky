Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
                                    expr: 0,
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
                                                        path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
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
                                                            7,
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
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            7,
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
                                                                path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
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
                                                                    7,
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
                                                    expr_idx: 0,
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
                                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
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
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                13,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            17,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                19,
                                            ),
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 0,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                10,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            12,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                20,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
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
                                                        9,
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
                                                        11,
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
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `hat`,
                                                    token_idx: TokenIdx(
                                                        18,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Eval {
                                            expr_idx: 6,
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
                                        expr_idx: 6,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 7,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
                                    expr: 4,
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
                                                        path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
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
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
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
                                                parent: 2,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        28,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `One`,
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `One`,
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
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            248,
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
                                                                path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
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
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 3,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `One`,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
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
                                                        parent: 2,
                                                        scope_resolution_token: ScopeResolutionToken(
                                                            TokenIdx(
                                                                28,
                                                            ),
                                                        ),
                                                        ident_token: Ok(
                                                            IdentToken {
                                                                ident: `One`,
                                                                token_idx: TokenIdx(
                                                                    29,
                                                                ),
                                                            },
                                                        ),
                                                        path: Ok(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `One`,
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
                                                    path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
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
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                34,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max_hole_ilen`,
                                                token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                41,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 0,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                32,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 2,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                36,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 3,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                38,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            39,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 4,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                42,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                43,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 5,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                44,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                52,
                                            ),
                                            items: [],
                                            rbox_token_idx: TokenIdx(
                                                53,
                                            ),
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 7,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                49,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 8,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            51,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 9,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                54,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_one_match`,
                                            token_idx: TokenIdx(
                                                56,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                57,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    58,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                60,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 47,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 12,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                59,
                                            ),
                                            ropd: 13,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
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
                                            owner: 16,
                                            dot_token_idx: TokenIdx(
                                                65,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max_row_span`,
                                                token_idx: TokenIdx(
                                                    66,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                70,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 15,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                63,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 17,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                67,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            68,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 18,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                71,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                72,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 19,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                73,
                                            ),
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
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                76,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max_row_span`,
                                                token_idx: TokenIdx(
                                                    77,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                79,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 48,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 22,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                78,
                                            ),
                                            ropd: 23,
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
                                            owner: 25,
                                            dot_token_idx: TokenIdx(
                                                83,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max_hole_ilen`,
                                                token_idx: TokenIdx(
                                                    84,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                86,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 49,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 26,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                85,
                                            ),
                                            ropd: 27,
                                        },
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
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 11,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 30,
                                            dot_token_idx: TokenIdx(
                                                94,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max_hole_ilen`,
                                                token_idx: TokenIdx(
                                                    95,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                97,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 50,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 31,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                96,
                                            ),
                                            ropd: 32,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 12,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                101,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 34,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                100,
                                            ),
                                            ropd: 35,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 13,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                106,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    107,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                109,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 38,
                                            lbox_token_idx: TokenIdx(
                                                108,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 39,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                110,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 14,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 41,
                                            dot_token_idx: TokenIdx(
                                                115,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    116,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                118,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 42,
                                            lbox_token_idx: TokenIdx(
                                                117,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 43,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                119,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 15,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 45,
                                            dot_token_idx: TokenIdx(
                                                124,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    125,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                127,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 46,
                                            lbox_token_idx: TokenIdx(
                                                126,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 47,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                128,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                130,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 49,
                                            be_token_idx: TokenIdx(
                                                131,
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
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_one_match`,
                                            token_idx: TokenIdx(
                                                135,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 51,
                                            dot_token_idx: TokenIdx(
                                                136,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    137,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                139,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 52,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 52,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                138,
                                            ),
                                            ropd: 53,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 16,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_one_match`,
                                            token_idx: TokenIdx(
                                                142,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 56,
                                            dot_token_idx: TokenIdx(
                                                143,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change_norm`,
                                                token_idx: TokenIdx(
                                                    144,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 57,
                                            dot_token_idx: TokenIdx(
                                                145,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                147,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                148,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                152,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 55,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                141,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 58,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                149,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            150,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 59,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                153,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                154,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 60,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                155,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 17,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                157,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Field {
                                            owner: 62,
                                            dot_token_idx: TokenIdx(
                                                160,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    161,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 18,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 63,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                158,
                                            ),
                                            ropd: 64,
                                        },
                                        SynExpr::Field {
                                            owner: 65,
                                            dot_token_idx: TokenIdx(
                                                164,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    165,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 66,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                162,
                                            ),
                                            ropd: 67,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                167,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 68,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                166,
                                            ),
                                            ropd: 69,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 20,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `Yes`,
                                                    },
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                175,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 72,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                176,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 73,
                                            dot_token_idx: TokenIdx(
                                                177,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    178,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                184,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 75,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                185,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 76,
                                            dot_token_idx: TokenIdx(
                                                186,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    187,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 74,
                                            dot_token_idx: TokenIdx(
                                                179,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    180,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                181,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                182,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 77,
                                            dot_token_idx: TokenIdx(
                                                188,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    189,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                190,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                191,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 78,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                183,
                                            ),
                                            ropd: 79,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 21,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 81,
                                            dot_token_idx: TokenIdx(
                                                194,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    195,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                197,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 55,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 82,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                196,
                                            ),
                                            ropd: 83,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 22,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 23,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                204,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 86,
                                            dot_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `top_k_row_span_sum`,
                                                token_idx: TokenIdx(
                                                    202,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                203,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 87,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                205,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 24,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 89,
                                            dot_token_idx: TokenIdx(
                                                208,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    209,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 25,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 91,
                                            dot_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    213,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 26,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 93,
                                            dot_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change_norm`,
                                                token_idx: TokenIdx(
                                                    217,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                221,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    12,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 85,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                199,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 88,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                206,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 90,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                210,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 92,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                214,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 94,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                218,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            219,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 95,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                222,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                223,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 96,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                224,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 27,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 28,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 99,
                                            dot_token_idx: TokenIdx(
                                                228,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                231,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 100,
                                            lbox_token_idx: TokenIdx(
                                                230,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 101,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                232,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 102,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                233,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 103,
                                            dot_token_idx: TokenIdx(
                                                234,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    235,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 29,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 105,
                                            dot_token_idx: TokenIdx(
                                                238,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                241,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 106,
                                            lbox_token_idx: TokenIdx(
                                                240,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 107,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                242,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 108,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                243,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 109,
                                            dot_token_idx: TokenIdx(
                                                244,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    245,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 110,
                                            dot_token_idx: TokenIdx(
                                                246,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    247,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                248,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                249,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                253,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 98,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                226,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 104,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                236,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 111,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                250,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            251,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 112,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                254,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                255,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 113,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                256,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                258,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 115,
                                            be_token_idx: TokenIdx(
                                                259,
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
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                263,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 117,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                264,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 118,
                                            dot_token_idx: TokenIdx(
                                                265,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    266,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                272,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 120,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                273,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 121,
                                            dot_token_idx: TokenIdx(
                                                274,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    275,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 119,
                                            dot_token_idx: TokenIdx(
                                                267,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    268,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                269,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                270,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 122,
                                            dot_token_idx: TokenIdx(
                                                276,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    277,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                278,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                279,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 123,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                271,
                                            ),
                                            ropd: 124,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                281,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 125,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                280,
                                            ),
                                            ropd: 126,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                285,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 128,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                286,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 129,
                                            dot_token_idx: TokenIdx(
                                                287,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    288,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                290,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 131,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                291,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 132,
                                            dot_token_idx: TokenIdx(
                                                292,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    293,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 133,
                                            dot_token_idx: TokenIdx(
                                                294,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    295,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                296,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                297,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                299,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 134,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                298,
                                            ),
                                            ropd: 135,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 130,
                                            lbox_token_idx: TokenIdx(
                                                289,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 136,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                300,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical`,
                                            token_idx: TokenIdx(
                                                304,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 138,
                                            dot_token_idx: TokenIdx(
                                                305,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    306,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                307,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                308,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                310,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 140,
                                            dot_token_idx: TokenIdx(
                                                311,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    312,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                314,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 141,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                313,
                                            ),
                                            ropd: 142,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 30,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                317,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 145,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                318,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 146,
                                            dot_token_idx: TokenIdx(
                                                319,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    320,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                322,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 148,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                323,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 149,
                                            dot_token_idx: TokenIdx(
                                                324,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    325,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                327,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 151,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                328,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 152,
                                            dot_token_idx: TokenIdx(
                                                329,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    330,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 153,
                                            dot_token_idx: TokenIdx(
                                                331,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    332,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                333,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                334,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                338,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 144,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                316,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 147,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                321,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 150,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                326,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 154,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                335,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            336,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 155,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                339,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                340,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 156,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                341,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 31,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                344,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 159,
                                            dot_token_idx: TokenIdx(
                                                345,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    346,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                347,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                348,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                350,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                354,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 161,
                                            dot_token_idx: TokenIdx(
                                                351,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    352,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 162,
                                            dot_token_idx: TokenIdx(
                                                355,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    356,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 163,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                353,
                                            ),
                                            ropd: 164,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                360,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 158,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                343,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 160,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                349,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 165,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                357,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            358,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 166,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                361,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                362,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 167,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                363,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `hat`,
                                            token_idx: TokenIdx(
                                                365,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 169,
                                            be_token_idx: TokenIdx(
                                                366,
                                            ),
                                            target: Ok(
                                                BeVariablesObelisk {
                                                    pattern_expr: 9,
                                                    variables: ArenaIdxRange(
                                                        9..10,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 32,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 33,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 172,
                                            dot_token_idx: TokenIdx(
                                                372,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    373,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                375,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 173,
                                            lbox_token_idx: TokenIdx(
                                                374,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 174,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                376,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 175,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                377,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 176,
                                            dot_token_idx: TokenIdx(
                                                378,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    379,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 34,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 178,
                                            dot_token_idx: TokenIdx(
                                                382,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    383,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                385,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 179,
                                            lbox_token_idx: TokenIdx(
                                                384,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 180,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                386,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 181,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                387,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 182,
                                            dot_token_idx: TokenIdx(
                                                388,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    389,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 35,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 184,
                                            dot_token_idx: TokenIdx(
                                                392,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    393,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                395,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 185,
                                            lbox_token_idx: TokenIdx(
                                                394,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 186,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                396,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 187,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                397,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 188,
                                            dot_token_idx: TokenIdx(
                                                398,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    399,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 189,
                                            dot_token_idx: TokenIdx(
                                                400,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    401,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                402,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                403,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                407,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 171,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                370,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 177,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                380,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 183,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                390,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 190,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                404,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            405,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 191,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                408,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                409,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 192,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                410,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_number_of_strokes`,
                                            token_idx: TokenIdx(
                                                412,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                414,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 194,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                413,
                                            ),
                                            ropd: 195,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                418,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 197,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                419,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 198,
                                            dot_token_idx: TokenIdx(
                                                420,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    421,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                423,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 200,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                424,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 201,
                                            dot_token_idx: TokenIdx(
                                                425,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    426,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 202,
                                            dot_token_idx: TokenIdx(
                                                427,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    428,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                429,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                430,
                                            ),
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 199,
                                            lbox_token_idx: TokenIdx(
                                                422,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 203,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                431,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_hat`,
                                            token_idx: TokenIdx(
                                                435,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 205,
                                            dot_token_idx: TokenIdx(
                                                436,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    437,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                438,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                439,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                443,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 207,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                444,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 208,
                                            dot_token_idx: TokenIdx(
                                                445,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    446,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                448,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 210,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                449,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 211,
                                            dot_token_idx: TokenIdx(
                                                450,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    451,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 212,
                                            dot_token_idx: TokenIdx(
                                                452,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    453,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                454,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                455,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                457,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 213,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                456,
                                            ),
                                            ropd: 214,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 209,
                                            lbox_token_idx: TokenIdx(
                                                447,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 215,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                458,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_feet`,
                                            token_idx: TokenIdx(
                                                462,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 217,
                                            dot_token_idx: TokenIdx(
                                                463,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    464,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                465,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                466,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 36,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_hat_dp`,
                                            token_idx: TokenIdx(
                                                469,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 220,
                                            dot_token_idx: TokenIdx(
                                                470,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    471,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_feet_dp`,
                                            token_idx: TokenIdx(
                                                473,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 13,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 222,
                                            dot_token_idx: TokenIdx(
                                                474,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    475,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                479,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 219,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                468,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 221,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                472,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 223,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                476,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            477,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 453,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 224,
                                                        separator: None,
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                480,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 225,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                481,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_number_of_strokes`,
                                            token_idx: TokenIdx(
                                                483,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                485,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 227,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                484,
                                            ),
                                            ropd: 228,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 37,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 230,
                                            dot_token_idx: TokenIdx(
                                                490,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    491,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 38,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 232,
                                            dot_token_idx: TokenIdx(
                                                496,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    497,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                501,
                                            ),
                                            current_symbol_idx: 14,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `b`,
                                            token_idx: TokenIdx(
                                                503,
                                            ),
                                            current_symbol_idx: 15,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 15,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 234,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                502,
                                            ),
                                            ropd: 235,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                507,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                509,
                                            ),
                                            current_symbol_idx: 14,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 237,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                508,
                                            ),
                                            ropd: 238,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                514,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 57,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                511,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                513,
                                            ),
                                            opd: 240,
                                        },
                                        SynExpr::Binary {
                                            lopd: 241,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                512,
                                            ),
                                            ropd: 242,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                516,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                518,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 58,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 244,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                517,
                                            ),
                                            ropd: 245,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 40,
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
                                                41..44,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        33,
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
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        48,
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
                                                        50,
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
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        64,
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
                                                        75,
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
                                                        82,
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
                                                        87,
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
                                                    88,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        89,
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
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        93,
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
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        105,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        159,
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
                                                        163,
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
                                                        169,
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
                                            parent: 19,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    170,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        171,
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
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        193,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        200,
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
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        211,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        225,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        227,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        237,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        315,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        342,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        369,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        371,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        381,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        391,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        467,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        489,
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
                                                        495,
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
                                                        519,
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
                                            parent: 39,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    520,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        521,
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
                                            condition: 28,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 20,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                },
                                                condition: Ok(
                                                    24,
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
                                                        0..1,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 29,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            condition: 54,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 61,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                            },
                                            condition: 70,
                                        },
                                        SynStmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    168,
                                                ),
                                            },
                                            result: 71,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    262,
                                                ),
                                            },
                                            condition: 127,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    282,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 7,
                                                    variables: ArenaIdxRange(
                                                        7..8,
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
                                                        284,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 137,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    301,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 8,
                                                    variables: ArenaIdxRange(
                                                        8..9,
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
                                                        303,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 139,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    309,
                                                ),
                                            },
                                            condition: 143,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 157,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 168,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 193,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    92,
                                                ),
                                            },
                                            condition: 33,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    98,
                                                ),
                                            },
                                            condition: 36,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    102,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
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
                                                        104,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 40,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    111,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 2,
                                                    variables: ArenaIdxRange(
                                                        2..3,
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
                                                        113,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 44,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 3,
                                                    variables: ArenaIdxRange(
                                                        3..4,
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
                                                        122,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 48,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                },
                                                condition: Ok(
                                                    50,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                133,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        4..8,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    172,
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
                                                        174,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 80,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    192,
                                                ),
                                            },
                                            condition: 84,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 97,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 114,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        257,
                                                    ),
                                                },
                                                condition: Ok(
                                                    116,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                261,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        8..14,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        364,
                                                    ),
                                                },
                                                condition: Ok(
                                                    170,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                368,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    411,
                                                ),
                                            },
                                            condition: 196,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    415,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 10,
                                                    variables: ArenaIdxRange(
                                                        10..11,
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
                                                        417,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 204,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    432,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 11,
                                                    variables: ArenaIdxRange(
                                                        11..12,
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
                                                        434,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 206,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    440,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 12,
                                                    variables: ArenaIdxRange(
                                                        12..13,
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
                                                        442,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 216,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    459,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 13,
                                                    variables: ArenaIdxRange(
                                                        13..14,
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
                                                        461,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 218,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 226,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    482,
                                                ),
                                            },
                                            condition: 229,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    486,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 14,
                                                    variables: ArenaIdxRange(
                                                        14..15,
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
                                                        488,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 231,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    492,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 15,
                                                    variables: ArenaIdxRange(
                                                        15..16,
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
                                                        494,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 233,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    498,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 16,
                                                    variables: ArenaIdxRange(
                                                        16..17,
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
                                                        500,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 236,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    504,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 17,
                                                    variables: ArenaIdxRange(
                                                        17..18,
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
                                                        506,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 239,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    510,
                                                ),
                                            },
                                            condition: 243,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    515,
                                                ),
                                            },
                                            condition: 246,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 247,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 6,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    45,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
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
                                                        47,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 10,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        55,
                                                    ),
                                                },
                                                condition: Ok(
                                                    14,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                61,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                SynElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    91,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    stmts: Ok(
                                                        ArenaIdxRange(
                                                            15..41,
                                                        ),
                                                    ),
                                                },
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
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        112,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `hat`,
                                                    token_idx: TokenIdx(
                                                        121,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_number_of_strokes`,
                                                    token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        260,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `long_vertical`,
                                                    token_idx: TokenIdx(
                                                        283,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        302,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        367,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_hat`,
                                                    token_idx: TokenIdx(
                                                        416,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_hat_dp`,
                                                    token_idx: TokenIdx(
                                                        433,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_feet`,
                                                    token_idx: TokenIdx(
                                                        441,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_feet_dp`,
                                                    token_idx: TokenIdx(
                                                        460,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        487,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `b`,
                                                    token_idx: TokenIdx(
                                                        493,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        499,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `d`,
                                                    token_idx: TokenIdx(
                                                        505,
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
                                            None,
                                            None,
                                            None,
                                            None,
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
                                        Let,
                                        Let,
                                        Let,
                                        Let,
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
                                                0,
                                            ),
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
                                            SynPatternSymbol::Atom(
                                                8,
                                            ),
                                            SynPatternSymbol::Atom(
                                                9,
                                            ),
                                            SynPatternSymbol::Atom(
                                                10,
                                            ),
                                            SynPatternSymbol::Atom(
                                                11,
                                            ),
                                            SynPatternSymbol::Atom(
                                                12,
                                            ),
                                            SynPatternSymbol::Atom(
                                                13,
                                            ),
                                            SynPatternSymbol::Atom(
                                                14,
                                            ),
                                            SynPatternSymbol::Atom(
                                                15,
                                            ),
                                            SynPatternSymbol::Atom(
                                                16,
                                            ),
                                            SynPatternSymbol::Atom(
                                                17,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `simp_one_match`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `downmost`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `upmost`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `hat`,
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
                                                `downmost_number_of_strokes`,
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                `long_vertical`,
                                                7,
                                            ),
                                        ],
                                        [
                                            (
                                                `long_vertical_dp`,
                                                8,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                9,
                                            ),
                                        ],
                                        [
                                            (
                                                `downmost_hat`,
                                                10,
                                            ),
                                        ],
                                        [
                                            (
                                                `downmost_hat_dp`,
                                                11,
                                            ),
                                        ],
                                        [
                                            (
                                                `downmost_feet`,
                                                12,
                                            ),
                                        ],
                                        [
                                            (
                                                `downmost_feet_dp`,
                                                13,
                                            ),
                                        ],
                                        [
                                            (
                                                `a`,
                                                14,
                                            ),
                                        ],
                                        [
                                            (
                                                `b`,
                                                15,
                                            ),
                                        ],
                                        [
                                            (
                                                `c`,
                                                16,
                                            ),
                                        ],
                                        [
                                            (
                                                `d`,
                                                17,
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
                                            None,
                                            None,
                                            None,
                                            None,
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
                                                    47,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `simp_one_match`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    104,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `downmost`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    113,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `upmost`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    122,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `hat`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    133,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            172,
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
                                                    174,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `downmost_number_of_strokes`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    261,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            364,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    284,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            364,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `long_vertical`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    303,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            364,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `long_vertical_dp`,
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    368,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            411,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    417,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `downmost_hat`,
                                                    pattern_symbol_idx: 10,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    434,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `downmost_hat_dp`,
                                                    pattern_symbol_idx: 11,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    442,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `downmost_feet`,
                                                    pattern_symbol_idx: 12,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    461,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `downmost_feet_dp`,
                                                    pattern_symbol_idx: 13,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    488,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `a`,
                                                    pattern_symbol_idx: 14,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    494,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `b`,
                                                    pattern_symbol_idx: 15,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    500,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `c`,
                                                    pattern_symbol_idx: 16,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    506,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            522,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `d`,
                                                    pattern_symbol_idx: 17,
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
                                        kind: EvalExpr,
                                        expr_idx: 6,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 10,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 20,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 28,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 29,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 33,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 36,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 40,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 44,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 48,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 54,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 61,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 70,
                                    },
                                    SynExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 71,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 80,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 84,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 97,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 114,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 127,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 137,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 139,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 143,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 157,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 168,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 193,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 196,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 204,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 206,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 216,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 218,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 226,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 229,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 231,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 233,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 236,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 239,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 243,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 246,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 247,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 248,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            526,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonObelisk {
                                    expr: 3,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    527,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
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
                                                    531,
                                                ),
                                                opd: 2,
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
                                                            528,
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
                                                            532,
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
                                                            525,
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
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `cc`,
                                                    0,
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
                                                        526,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            8,
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
                                                                path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            527,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
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
                                                            531,
                                                        ),
                                                        opd: 2,
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
                                                                    528,
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
                                                                    532,
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
                                                                    525,
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
                                                            0,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            0,
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
                                                                526,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
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
                                                    path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
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
                                                537,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                538,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    539,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                540,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                541,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                543,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                544,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    545,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                547,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 59,
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
                                                546,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                548,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                549,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    550,
                                                ),
                                            },
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    534,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
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
                                                        536,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    542,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 7,
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
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        535,
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
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            SynPatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `dp`,
                                                0,
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    536,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            551,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `dp`,
                                                    pattern_symbol_idx: 0,
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 7,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 8,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            555,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonObelisk {
                                    expr: 3,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    556,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
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
                                                    560,
                                                ),
                                                opd: 2,
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
                                                            557,
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
                                                            561,
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
                                                            554,
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
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `cc`,
                                                    0,
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
                                                        555,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            10,
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
                                                                path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            556,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
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
                                                            560,
                                                        ),
                                                        opd: 2,
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
                                                                    557,
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
                                                                    561,
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
                                                                    554,
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
                                                            0,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            0,
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
                                                                555,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
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
                                                    path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
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
                                                566,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                567,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    568,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                569,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                570,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                572,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                573,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    574,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                576,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                575,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                578,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                579,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    580,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                581,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                582,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                583,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    584,
                                                ),
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                577,
                                            ),
                                            opd: 8,
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    563,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
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
                                                        565,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    571,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 9,
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
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        564,
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
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            SynPatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `dp`,
                                                0,
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    565,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            585,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `dp`,
                                                    pattern_symbol_idx: 0,
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 9,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 10,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            589,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonObelisk {
                                    expr: 3,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    590,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
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
                                                    594,
                                                ),
                                                opd: 2,
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
                                                            591,
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
                                                            595,
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
                                                            588,
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
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `cc`,
                                                    0,
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
                                                        589,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            16,
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
                                                                path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            590,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
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
                                                            594,
                                                        ),
                                                        opd: 2,
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
                                                                    591,
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
                                                                    595,
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
                                                                    588,
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
                                                            0,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            0,
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
                                                                589,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
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
                                                    path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
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
                                                600,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                601,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    602,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                603,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                604,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                606,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                607,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    608,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                610,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                609,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                612,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                613,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    614,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                616,
                                            ),
                                            Literal::Float(
                                                Unspecified(
                                                    UnspecifiedFloatLiteral(
                                                        Id {
                                                            value: 62,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 7,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                615,
                                            ),
                                            ropd: 8,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                618,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                619,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    620,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                622,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                617,
                                            ),
                                            opd: 11,
                                        },
                                        SynExpr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                623,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    624,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 13,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                621,
                                            ),
                                            ropd: 14,
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..4,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    597,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
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
                                                        599,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    605,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    611,
                                                ),
                                            },
                                            condition: 9,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 15,
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
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        598,
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
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            SynPatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `dp`,
                                                0,
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    599,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            625,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `dp`,
                                                    pattern_symbol_idx: 0,
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 9,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 15,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 16,
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