Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            return_ty: Some(
                                ReturnTypeExprBeforeEq {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
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
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
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
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
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
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
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
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
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
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            return_ty: Some(
                                ReturnTypeExprBeforeEq {
                                    expr: 4,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
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
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
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
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
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
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
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
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
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
                                    pattern_expr_region: PatternSynExprRegion {
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
                            235,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
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
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
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
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
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
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
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
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
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
                                            pattern_expr_region: PatternSynExprRegion {
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                                    value: 448,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                Unspecified,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 6,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                                    value: 448,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                Unspecified,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                Unspecified,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                Unspecified,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                Unspecified,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                BeVariablesPattern {
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
                                                Unspecified,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                                    value: 448,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                Unspecified,
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
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
                                                Unspecified,
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
                                        SynExpr::Field {
                                            owner: 72,
                                            dot_token_idx: TokenIdx(
                                                176,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    177,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                181,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 74,
                                            dot_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 73,
                                            dot_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    179,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 75,
                                            dot_token_idx: TokenIdx(
                                                184,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    185,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 76,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                180,
                                            ),
                                            ropd: 77,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 21,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 79,
                                            dot_token_idx: TokenIdx(
                                                188,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    189,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                191,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 80,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                190,
                                            ),
                                            ropd: 81,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 22,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 23,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                198,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 84,
                                            dot_token_idx: TokenIdx(
                                                195,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `top_k_row_span_sum`,
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                197,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 85,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                199,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 24,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 87,
                                            dot_token_idx: TokenIdx(
                                                202,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    203,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 25,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 89,
                                            dot_token_idx: TokenIdx(
                                                206,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    207,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 26,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 91,
                                            dot_token_idx: TokenIdx(
                                                210,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change_norm`,
                                                token_idx: TokenIdx(
                                                    211,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                215,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    12,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 83,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                193,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 86,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                200,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 88,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                204,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 90,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                208,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 92,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                212,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            213,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 448,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 93,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                216,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                217,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 94,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                218,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 27,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 28,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 97,
                                            dot_token_idx: TokenIdx(
                                                222,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    223,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                225,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 98,
                                            lbox_token_idx: TokenIdx(
                                                224,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 99,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                226,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 100,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                227,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 101,
                                            dot_token_idx: TokenIdx(
                                                228,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 29,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 103,
                                            dot_token_idx: TokenIdx(
                                                232,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    233,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                235,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 104,
                                            lbox_token_idx: TokenIdx(
                                                234,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 105,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                236,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 106,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                237,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 107,
                                            dot_token_idx: TokenIdx(
                                                238,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 108,
                                            dot_token_idx: TokenIdx(
                                                240,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    241,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                242,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                243,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                247,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 96,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                220,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 102,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                230,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 109,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                244,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            245,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 448,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 110,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                248,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                249,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 111,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                250,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                252,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 113,
                                            be_token_idx: TokenIdx(
                                                253,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
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
                                                257,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 115,
                                            dot_token_idx: TokenIdx(
                                                258,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    259,
                                                ),
                                            },
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
                                        SynExpr::Field {
                                            owner: 117,
                                            dot_token_idx: TokenIdx(
                                                264,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    265,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 116,
                                            dot_token_idx: TokenIdx(
                                                260,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    261,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 118,
                                            dot_token_idx: TokenIdx(
                                                266,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    267,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 119,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                262,
                                            ),
                                            ropd: 120,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                269,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 121,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                268,
                                            ),
                                            ropd: 122,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                273,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 124,
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
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                277,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 126,
                                            dot_token_idx: TokenIdx(
                                                278,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    279,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 127,
                                            dot_token_idx: TokenIdx(
                                                280,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    281,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                283,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 128,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                282,
                                            ),
                                            ropd: 129,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 125,
                                            lbox_token_idx: TokenIdx(
                                                276,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 130,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                284,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical`,
                                            token_idx: TokenIdx(
                                                288,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 132,
                                            dot_token_idx: TokenIdx(
                                                289,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    290,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                291,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                292,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                294,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 134,
                                            dot_token_idx: TokenIdx(
                                                295,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    296,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                298,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 135,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                297,
                                            ),
                                            ropd: 136,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 30,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                301,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 139,
                                            dot_token_idx: TokenIdx(
                                                302,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    303,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                305,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 141,
                                            dot_token_idx: TokenIdx(
                                                306,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    307,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                309,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 143,
                                            dot_token_idx: TokenIdx(
                                                310,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    311,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 144,
                                            dot_token_idx: TokenIdx(
                                                312,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    313,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                314,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                315,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                319,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 138,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                300,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 140,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                304,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 142,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                308,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 145,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                316,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            317,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 448,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 146,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                320,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                321,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 147,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                322,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 31,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                325,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 150,
                                            dot_token_idx: TokenIdx(
                                                326,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    327,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                328,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                329,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                331,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                335,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 152,
                                            dot_token_idx: TokenIdx(
                                                332,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    333,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 153,
                                            dot_token_idx: TokenIdx(
                                                336,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    337,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 154,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                334,
                                            ),
                                            ropd: 155,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                341,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 149,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                324,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 151,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                330,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 156,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                338,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            339,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 448,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 157,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                342,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                343,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 158,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                344,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `hat`,
                                            token_idx: TokenIdx(
                                                346,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 160,
                                            be_token_idx: TokenIdx(
                                                347,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
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
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 33,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 163,
                                            dot_token_idx: TokenIdx(
                                                353,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    354,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                356,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 164,
                                            lbox_token_idx: TokenIdx(
                                                355,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 165,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                357,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 166,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                358,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 167,
                                            dot_token_idx: TokenIdx(
                                                359,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    360,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 34,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 169,
                                            dot_token_idx: TokenIdx(
                                                363,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    364,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                366,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 170,
                                            lbox_token_idx: TokenIdx(
                                                365,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 171,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                367,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 172,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                368,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 173,
                                            dot_token_idx: TokenIdx(
                                                369,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    370,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 35,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 175,
                                            dot_token_idx: TokenIdx(
                                                373,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    374,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                376,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 176,
                                            lbox_token_idx: TokenIdx(
                                                375,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 177,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                377,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 178,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                378,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 179,
                                            dot_token_idx: TokenIdx(
                                                379,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    380,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 180,
                                            dot_token_idx: TokenIdx(
                                                381,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    382,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                383,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                384,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                388,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 162,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                351,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 168,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                361,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 174,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                371,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 181,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                385,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            386,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 448,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 182,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                389,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                390,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 183,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                391,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_number_of_strokes`,
                                            token_idx: TokenIdx(
                                                393,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                395,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 185,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                394,
                                            ),
                                            ropd: 186,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                399,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 188,
                                            dot_token_idx: TokenIdx(
                                                400,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    401,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                403,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 190,
                                            dot_token_idx: TokenIdx(
                                                404,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    405,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 191,
                                            dot_token_idx: TokenIdx(
                                                406,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    407,
                                                ),
                                            },
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 189,
                                            lbox_token_idx: TokenIdx(
                                                402,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 192,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                408,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_hat`,
                                            token_idx: TokenIdx(
                                                412,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 194,
                                            dot_token_idx: TokenIdx(
                                                413,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    414,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                415,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                416,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                420,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 196,
                                            dot_token_idx: TokenIdx(
                                                421,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    422,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                424,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 198,
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
                                        SynExpr::Field {
                                            owner: 199,
                                            dot_token_idx: TokenIdx(
                                                427,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    428,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                430,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 200,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                429,
                                            ),
                                            ropd: 201,
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 197,
                                            lbox_token_idx: TokenIdx(
                                                423,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 202,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                431,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_feet`,
                                            token_idx: TokenIdx(
                                                435,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 204,
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
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 36,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_hat_dp`,
                                            token_idx: TokenIdx(
                                                442,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 207,
                                            dot_token_idx: TokenIdx(
                                                443,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    444,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_feet_dp`,
                                            token_idx: TokenIdx(
                                                446,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 13,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 209,
                                            dot_token_idx: TokenIdx(
                                                447,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    448,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                452,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 206,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                441,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 208,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                445,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 210,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                449,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            450,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 448,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 211,
                                                        separator: None,
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                453,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 212,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                454,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `downmost_number_of_strokes`,
                                            token_idx: TokenIdx(
                                                456,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                458,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 214,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                457,
                                            ),
                                            ropd: 215,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 37,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 217,
                                            dot_token_idx: TokenIdx(
                                                463,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    464,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 38,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 219,
                                            dot_token_idx: TokenIdx(
                                                469,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    470,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                474,
                                            ),
                                            current_symbol_idx: 14,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `b`,
                                            token_idx: TokenIdx(
                                                476,
                                            ),
                                            current_symbol_idx: 15,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 15,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 221,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                475,
                                            ),
                                            ropd: 222,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                480,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                482,
                                            ),
                                            current_symbol_idx: 14,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 224,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                481,
                                            ),
                                            ropd: 225,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                487,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                484,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                486,
                                            ),
                                            opd: 227,
                                        },
                                        SynExpr::Binary {
                                            lopd: 228,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                485,
                                            ),
                                            ropd: 229,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                489,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                491,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 231,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                490,
                                            ),
                                            ropd: 232,
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
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
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
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
                                                        187,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        299,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        350,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        352,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        362,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `one_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        372,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        440,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        462,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        468,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `OneVsAll`,
                                                    token_idx: TokenIdx(
                                                        492,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Subitem {
                                            parent: 39,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    493,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        494,
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
                                        },
                                        SynStmt::IfElse {
                                            if_branch: IfBranch {
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
                                                block: Ok(
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
                                                    256,
                                                ),
                                            },
                                            condition: 123,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    270,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        272,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 131,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    285,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        287,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 133,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    293,
                                                ),
                                            },
                                            condition: 137,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 148,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 159,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 184,
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
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                            if_branch: IfBranch {
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
                                                block: Ok(
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
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                            initial_value: 78,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    186,
                                                ),
                                            },
                                            condition: 82,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 95,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 112,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        251,
                                                    ),
                                                },
                                                condition: Ok(
                                                    114,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                255,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        8..14,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        345,
                                                    ),
                                                },
                                                condition: Ok(
                                                    161,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                349,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
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
                                                    392,
                                                ),
                                            },
                                            condition: 187,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    396,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        398,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 193,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    409,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        411,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 195,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    417,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        419,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 203,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    432,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        434,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 205,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 213,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    455,
                                                ),
                                            },
                                            condition: 216,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    459,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        461,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 218,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    465,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        467,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 220,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    471,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        473,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 223,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    477,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        479,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 226,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    483,
                                                ),
                                            },
                                            condition: 230,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    488,
                                                ),
                                            },
                                            condition: 233,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 234,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 6,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    45,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                            if_branch: IfBranch {
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
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
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
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            15..41,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        112,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `hat`,
                                                    token_idx: TokenIdx(
                                                        121,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_number_of_strokes`,
                                                    token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `long_vertical`,
                                                    token_idx: TokenIdx(
                                                        271,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        286,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        348,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_hat`,
                                                    token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_hat_dp`,
                                                    token_idx: TokenIdx(
                                                        410,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_feet`,
                                                    token_idx: TokenIdx(
                                                        418,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downmost_feet_dp`,
                                                    token_idx: TokenIdx(
                                                        433,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        460,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `b`,
                                                    token_idx: TokenIdx(
                                                        466,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        472,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `d`,
                                                    token_idx: TokenIdx(
                                                        478,
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
                                            PatternSynSymbol::Atom(
                                                0,
                                            ),
                                            PatternSynSymbol::Atom(
                                                1,
                                            ),
                                            PatternSynSymbol::Atom(
                                                2,
                                            ),
                                            PatternSynSymbol::Atom(
                                                3,
                                            ),
                                            PatternSynSymbol::Atom(
                                                4,
                                            ),
                                            PatternSynSymbol::Atom(
                                                5,
                                            ),
                                            PatternSynSymbol::Atom(
                                                6,
                                            ),
                                            PatternSynSymbol::Atom(
                                                7,
                                            ),
                                            PatternSynSymbol::Atom(
                                                8,
                                            ),
                                            PatternSynSymbol::Atom(
                                                9,
                                            ),
                                            PatternSynSymbol::Atom(
                                                10,
                                            ),
                                            PatternSynSymbol::Atom(
                                                11,
                                            ),
                                            PatternSynSymbol::Atom(
                                                12,
                                            ),
                                            PatternSynSymbol::Atom(
                                                13,
                                            ),
                                            PatternSynSymbol::Atom(
                                                14,
                                            ),
                                            PatternSynSymbol::Atom(
                                                15,
                                            ),
                                            PatternSynSymbol::Atom(
                                                16,
                                            ),
                                            PatternSynSymbol::Atom(
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
                                                            495,
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
                                                            495,
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
                                                            495,
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
                                                            495,
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
                                                            495,
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
                                                    255,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            345,
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
                                                    272,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            345,
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
                                                    287,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            345,
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
                                                    349,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            392,
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
                                                    398,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                                    411,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                                    419,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                                    434,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                                    461,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                                    467,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                                    473,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                                    479,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            495,
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
                                        expr_idx: 78,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 82,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 95,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 112,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 123,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 131,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 133,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 137,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 148,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 159,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 184,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 187,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 193,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 195,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 203,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 205,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 213,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 216,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 218,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 220,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 223,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 226,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 230,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 233,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 234,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 235,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                            template_parameters: [],
                            parenic_parameters: [
                                SpecificParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            499,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 3,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
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
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    500,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    504,
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
                                                            501,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            505,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternSynExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            498,
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
                                                PatternSynSymbol::Atom(
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
                                                        499,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
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
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            500,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            504,
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
                                                                    501,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    505,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternSynExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    498,
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
                                                        PatternSynSymbol::Atom(
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
                                                                499,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
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
                                                510,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                511,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    512,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                513,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                514,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                516,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                517,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    518,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                520,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                519,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                521,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                522,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    523,
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
                                                    507,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        509,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    515,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 7,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        508,
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
                                            PatternSynSymbol::Atom(
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
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
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
                                                    509,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            524,
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
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                            template_parameters: [],
                            parenic_parameters: [
                                SpecificParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            528,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 3,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
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
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    529,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    533,
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
                                                            530,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            534,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternSynExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            527,
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
                                                PatternSynSymbol::Atom(
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
                                                        528,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
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
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            529,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            533,
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
                                                                    530,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    534,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternSynExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    527,
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
                                                        PatternSynSymbol::Atom(
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
                                                                528,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
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
                                                539,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                540,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    541,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                542,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                543,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                545,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                546,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    547,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                549,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                548,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                551,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                552,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    553,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                554,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                555,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                556,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    557,
                                                ),
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                550,
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
                                                    536,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        538,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    544,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 9,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        537,
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
                                            PatternSynSymbol::Atom(
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
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
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
                                                    538,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            558,
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
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                            template_parameters: [],
                            parenic_parameters: [
                                SpecificParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            562,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 3,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
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
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    563,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    567,
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
                                                            564,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            568,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternSynExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            561,
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
                                                PatternSynSymbol::Atom(
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
                                                        562,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
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
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            563,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            567,
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
                                                                    564,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    568,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternSynExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    561,
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
                                                        PatternSynSymbol::Atom(
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
                                                                562,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenicRegularParameter {
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
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
                                                573,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                574,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    575,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                576,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                577,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                579,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                580,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    581,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                583,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                582,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                585,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                586,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    587,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                589,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 7,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                588,
                                            ),
                                            ropd: 8,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                591,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                592,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    593,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                595,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                590,
                                            ),
                                            opd: 11,
                                        },
                                        SynExpr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                596,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    597,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 13,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                594,
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
                                                    570,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        572,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    578,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    584,
                                                ),
                                            },
                                            condition: 9,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 15,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        571,
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
                                            PatternSynSymbol::Atom(
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
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
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
                                                    572,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            598,
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