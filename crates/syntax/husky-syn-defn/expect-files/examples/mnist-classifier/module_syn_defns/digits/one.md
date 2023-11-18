Ok(
    [
        ItemSynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        decl: ValFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
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
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
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
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `FermiMatchResult`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
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
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                8,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: SynNodeRegionPath::Decl(
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
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 1,
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
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `FermiMatchResult`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
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
                                                            syn_expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
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
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 5,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 3,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            syn_expr_idx: 4,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            syn_expr_idx: 5,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                },
                                                SynExprData::FunctionApplicationOrCall {
                                                    function: 1,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 2,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            syn_expr_idx: 6,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_concave_components`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `downmost`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `upmost`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `hat`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Eval {
                                                    expr_idx: 7,
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
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 7,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 8,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        ItemSynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                        decl: ValFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 5,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
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
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `OneVsAll`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `MnistLabel`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `MnistLabel`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 3,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        8,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `One`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
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
                                            syn_expr_idx: 5,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                249,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: SynNodeRegionPath::Decl(
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
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 4,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::TypeVariant(
                                                                        TypeVariantPath {
                                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                            ident: `One`,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::ExplicitApplication {
                                                                function_expr_idx: 3,
                                                                argument_expr_idx: 4,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `OneVsAll`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `MnistLabel`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `MnistLabel`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Subitem {
                                                                parent: 3,
                                                                colon_colon_token: ColonColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        8,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentRegionalToken {
                                                                        ident: `One`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            9,
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
                                                            syn_expr_idx: 5,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
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
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 2,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max_hole_ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 1,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 3,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 4,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        8,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 5,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        12,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 6,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 5,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                },
                                                SynExprData::FunctionApplicationOrCall {
                                                    function: 8,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 9,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    21,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            syn_expr_idx: 10,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `simp_one_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        27,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 47,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 13,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    ropd: 14,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 6,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 7,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 17,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max_row_span`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            36,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        40,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 16,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 18,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        37,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    38,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 19,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        41,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 20,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 8,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 22,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max_row_span`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        49,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 48,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 23,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        48,
                                                    ),
                                                    ropd: 24,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 9,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 26,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        53,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max_hole_ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            54,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        56,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 49,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 27,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        55,
                                                    ),
                                                    ropd: 28,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 11,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ident: `Yes`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 12,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 31,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        64,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max_hole_ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            65,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        67,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 50,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 32,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        66,
                                                    ),
                                                    ropd: 33,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 13,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        71,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 51,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 35,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        70,
                                                    ),
                                                    ropd: 36,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 15,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 38,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        76,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            77,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        79,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 39,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        78,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 40,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        80,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 17,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 42,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        85,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            86,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        88,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 43,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        87,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 44,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        89,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 19,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 46,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        94,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            95,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        97,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 47,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        96,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 48,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        98,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        100,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Be {
                                                    src: 50,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        101,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 5,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                5..6,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `simp_one_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        105,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 52,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        106,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            107,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        109,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 53,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        108,
                                                    ),
                                                    ropd: 54,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 20,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `simp_one_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        112,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 57,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        113,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            114,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 58,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        115,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `abs`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            116,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        117,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        118,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        122,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 56,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        111,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 59,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        119,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    120,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 60,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        123,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        124,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 61,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        125,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 21,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        127,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 53,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Field {
                                                    owner: 63,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        130,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            131,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 22,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 64,
                                                    opr: Closed(
                                                        Mul,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        128,
                                                    ),
                                                    ropd: 65,
                                                },
                                                SynExprData::Field {
                                                    owner: 66,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        134,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            135,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Binary {
                                                    lopd: 67,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        132,
                                                    ),
                                                    ropd: 68,
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        137,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 54,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 69,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        136,
                                                    ),
                                                    ropd: 70,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 24,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ident: `Yes`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        145,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 73,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        146,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 74,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        147,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            148,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        154,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 76,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        155,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 77,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        156,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            157,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 75,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        149,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            150,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        151,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        152,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 78,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        158,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            159,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        160,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        161,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 79,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        153,
                                                    ),
                                                    ropd: 80,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 25,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 82,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        164,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            165,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        167,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 55,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 83,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        166,
                                                    ),
                                                    ropd: 84,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 26,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 27,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        174,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 87,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        171,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `top_k_row_span_sum`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            172,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        173,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 88,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        175,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 28,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 90,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        178,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            179,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 29,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 92,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        182,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `rel_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            183,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 30,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 94,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        186,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            187,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        191,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            12,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 86,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        169,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 89,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        176,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 91,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        180,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 93,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        184,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 95,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        188,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    189,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 96,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        192,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        193,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 97,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        194,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 31,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 32,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 100,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        198,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            199,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        201,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 101,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        200,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 102,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        202,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 103,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        203,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 104,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        204,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `rel_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            205,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 33,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 106,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        208,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            209,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        211,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 107,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        210,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 108,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        212,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 109,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        213,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 110,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        214,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 111,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        216,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `abs`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            217,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        218,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        219,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        223,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 99,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        196,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 105,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        206,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 112,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        220,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    221,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 113,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        224,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        225,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 114,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        226,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        228,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Be {
                                                    src: 116,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        229,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 7,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                7..8,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        233,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 118,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        234,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 119,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        235,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            236,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        242,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 121,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        243,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 122,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        244,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            245,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 120,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        237,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            238,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        239,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        240,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 123,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        246,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            247,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        248,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        249,
                                                    ),
                                                },
                                                SynExprData::Binary {
                                                    lopd: 124,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        241,
                                                    ),
                                                    ropd: 125,
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        251,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 126,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        250,
                                                    ),
                                                    ropd: 127,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        255,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 129,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        256,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 130,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        257,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            258,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        260,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 132,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        261,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 133,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        262,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            263,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 134,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        264,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            265,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        266,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        267,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        269,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 135,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        268,
                                                    ),
                                                    ropd: 136,
                                                },
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 131,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        259,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 137,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        270,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `long_vertical`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        274,
                                                    ),
                                                    current_syn_symbol_idx: 8,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 139,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        275,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            276,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        277,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        278,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `long_vertical_dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        280,
                                                    ),
                                                    current_syn_symbol_idx: 9,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 141,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        281,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            282,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        284,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 56,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 142,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        283,
                                                    ),
                                                    ropd: 143,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 34,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        287,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 146,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        288,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 147,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        289,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            290,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        292,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 149,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        293,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 150,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        294,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `rel_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            295,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        297,
                                                    ),
                                                    current_syn_symbol_idx: 3,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 152,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        298,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 153,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        299,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            300,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 154,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        301,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `abs`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            302,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        303,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        304,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        308,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 145,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        286,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 148,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        291,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 151,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        296,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 155,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        305,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    306,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 156,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        309,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        310,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 157,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        311,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 35,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `long_vertical_dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        314,
                                                    ),
                                                    current_syn_symbol_idx: 9,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 160,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        315,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            316,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        317,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        318,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `long_vertical_dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        320,
                                                    ),
                                                    current_syn_symbol_idx: 9,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `long_vertical_dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        324,
                                                    ),
                                                    current_syn_symbol_idx: 9,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 162,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        321,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            322,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 163,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        325,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            326,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Binary {
                                                    lopd: 164,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        323,
                                                    ),
                                                    ropd: 165,
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        330,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 159,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        313,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 161,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        319,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 166,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        327,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    328,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 167,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        331,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        332,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 168,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        333,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `hat`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        335,
                                                    ),
                                                    current_syn_symbol_idx: 4,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExprData::Be {
                                                    src: 170,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        336,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 10,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                10..11,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 36,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 37,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 173,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        342,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            343,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        345,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 174,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        344,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 175,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        346,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 176,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        347,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 177,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        348,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            349,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 38,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 179,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        352,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            353,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        355,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 180,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        354,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 181,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        356,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 182,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        357,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 183,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        358,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `rel_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            359,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 39,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 185,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        362,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            363,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        365,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 186,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        364,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 187,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        366,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 188,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        367,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 189,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        368,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            369,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 190,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        370,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `abs`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            371,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        372,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        373,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        377,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 172,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        340,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 178,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        350,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 184,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        360,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 191,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        374,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    375,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 192,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        378,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        379,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 193,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        380,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost_number_of_strokes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        382,
                                                    ),
                                                    current_syn_symbol_idx: 6,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        384,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 195,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        383,
                                                    ),
                                                    ropd: 196,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        388,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 198,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        389,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 199,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        390,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            391,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        393,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 201,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        394,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 202,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        395,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            396,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 203,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        397,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            398,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        399,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        400,
                                                    ),
                                                },
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 200,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        392,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 204,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        401,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost_hat`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        405,
                                                    ),
                                                    current_syn_symbol_idx: 11,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 206,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        406,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            407,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        408,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        409,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        413,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 208,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        414,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 209,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        415,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            416,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        418,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 211,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        419,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 212,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        420,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strokes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            421,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 213,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        422,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            423,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        424,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        425,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        427,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 214,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        426,
                                                    ),
                                                    ropd: 215,
                                                },
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 210,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        417,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 216,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        428,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost_feet`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        432,
                                                    ),
                                                    current_syn_symbol_idx: 13,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 218,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        433,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            434,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        435,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        436,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 40,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost_hat_dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        439,
                                                    ),
                                                    current_syn_symbol_idx: 12,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 221,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        440,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            441,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost_feet_dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        443,
                                                    ),
                                                    current_syn_symbol_idx: 14,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 14,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 223,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        444,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            445,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        449,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 220,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        438,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 222,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        442,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 224,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        446,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    447,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 225,
                                                                separator: None,
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        450,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 226,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        451,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `downmost_number_of_strokes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        453,
                                                    ),
                                                    current_syn_symbol_idx: 6,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        455,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 228,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        454,
                                                    ),
                                                    ropd: 229,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 41,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 231,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        460,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            461,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 42,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 233,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        466,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            467,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `a`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        471,
                                                    ),
                                                    current_syn_symbol_idx: 15,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 15,
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `b`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        473,
                                                    ),
                                                    current_syn_symbol_idx: 16,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                SynExprData::Binary {
                                                    lopd: 235,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        472,
                                                    ),
                                                    ropd: 236,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `c`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        477,
                                                    ),
                                                    current_syn_symbol_idx: 17,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 17,
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `a`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        479,
                                                    ),
                                                    current_syn_symbol_idx: 15,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 15,
                                                    },
                                                },
                                                SynExprData::Binary {
                                                    lopd: 238,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        478,
                                                    ),
                                                    ropd: 239,
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        484,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 57,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `c`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        481,
                                                    ),
                                                    current_syn_symbol_idx: 17,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 17,
                                                    },
                                                },
                                                SynExprData::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        483,
                                                    ),
                                                    opd: 241,
                                                },
                                                SynExprData::Binary {
                                                    lopd: 242,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        482,
                                                    ),
                                                    ropd: 243,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `c`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        486,
                                                    ),
                                                    current_syn_symbol_idx: 17,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 17,
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        488,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 58,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 245,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        487,
                                                    ),
                                                    ropd: 246,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 44,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ident: `Yes`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        42..45,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_concave_components`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                20,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                32,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                45,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                52,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                57,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Subitem {
                                                    parent: 10,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            58,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                59,
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
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                63,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                69,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `downmost`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                73,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                75,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `upmost`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                82,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                84,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `hat`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                91,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                93,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                110,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                129,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                133,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                139,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Subitem {
                                                    parent: 23,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            140,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                141,
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
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                163,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                168,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                170,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                177,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                181,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                185,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                195,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                197,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
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
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                285,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                312,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                339,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                341,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                351,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `one_fermi_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                361,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                437,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                459,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                465,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                489,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Subitem {
                                                    parent: 43,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            490,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                491,
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
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            51,
                                                        ),
                                                    },
                                                    condition: 29,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 21,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                44,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            25,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    50,
                                                                ),
                                                            },
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 30,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            104,
                                                        ),
                                                    },
                                                    condition: 55,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 62,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            126,
                                                        ),
                                                    },
                                                    condition: 71,
                                                },
                                                SynStmtData::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                    result: 72,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            232,
                                                        ),
                                                    },
                                                    condition: 128,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            252,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 8,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                254,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 138,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            271,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 9,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                9..10,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                273,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 140,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            279,
                                                        ),
                                                    },
                                                    condition: 144,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 158,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 169,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 194,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                    condition: 34,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            68,
                                                        ),
                                                    },
                                                    condition: 37,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            72,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 2,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                74,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 41,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            81,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 3,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                83,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 45,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            90,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 4,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                92,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 49,
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                99,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            51,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    103,
                                                                ),
                                                            },
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            5..9,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            142,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 6,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                6..7,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                144,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 81,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            162,
                                                        ),
                                                    },
                                                    condition: 85,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 98,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 115,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                227,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            117,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    231,
                                                                ),
                                                            },
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            9..15,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                334,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            171,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    338,
                                                                ),
                                                            },
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            15..16,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            381,
                                                        ),
                                                    },
                                                    condition: 197,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            385,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 11,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                387,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 205,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            402,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 12,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                404,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 207,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            410,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 13,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                412,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 217,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            429,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 14,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                431,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 219,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 227,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            452,
                                                        ),
                                                    },
                                                    condition: 230,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            456,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 15,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                458,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 232,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            462,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 16,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                464,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 234,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            468,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 17,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                470,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 237,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            474,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 18,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                18..19,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                476,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 240,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            480,
                                                        ),
                                                    },
                                                    condition: 244,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            485,
                                                        ),
                                                    },
                                                    condition: 247,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 248,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 7,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 11,
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                25,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            15,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    31,
                                                                ),
                                                            },
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            2..5,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: Some(
                                                        SynElseBranch {
                                                            else_token: ElseRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    60,
                                                                ),
                                                            },
                                                            eol_colon_token: Ok(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        61,
                                                                    ),
                                                                },
                                                            ),
                                                            stmts: ArenaIdxRange(
                                                                16..42,
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
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `simp_one_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `downmost`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                73,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `upmost`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                82,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `hat`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                91,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                102,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `downmost_number_of_strokes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                143,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                230,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `long_vertical`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                253,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `long_vertical_dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                272,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                337,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `downmost_hat`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                386,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `downmost_hat_dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                403,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `downmost_feet`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                411,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `downmost_feet_dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                430,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `a`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                457,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `b`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                463,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `c`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                469,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `d`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                475,
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
                                                    SynPatternSymbol::Atom(
                                                        18,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `simp_one_match`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `upmost`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `hat`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `none`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_number_of_strokes`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `long_vertical`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `long_vertical_dp`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_hat`,
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_hat_dp`,
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_feet`,
                                                        13,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_feet_dp`,
                                                        14,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        15,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `b`,
                                                        16,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `c`,
                                                        17,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `d`,
                                                        18,
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
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `simp_one_match`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            74,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `downmost`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            83,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `upmost`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            92,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `hat`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            103,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    142,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            144,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `downmost_number_of_strokes`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            231,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    334,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            254,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    334,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `long_vertical`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            273,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    334,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `long_vertical_dp`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            338,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    381,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            387,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `downmost_hat`,
                                                            pattern_symbol_idx: 11,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            404,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `downmost_hat_dp`,
                                                            pattern_symbol_idx: 12,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            412,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `downmost_feet`,
                                                            pattern_symbol_idx: 13,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            431,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `downmost_feet_dp`,
                                                            pattern_symbol_idx: 14,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            458,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 15,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            464,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `b`,
                                                            pattern_symbol_idx: 16,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            470,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `c`,
                                                            pattern_symbol_idx: 17,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            476,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    492,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `d`,
                                                            pattern_symbol_idx: 18,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 2,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 3,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 4,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Be,
                                                syn_pattern_expr_idx: 5,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 6,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Be,
                                                syn_pattern_expr_idx: 7,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 8,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 9,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Be,
                                                syn_pattern_expr_idx: 10,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 11,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 12,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 13,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 14,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 15,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 16,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 17,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 18,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 7,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 11,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 21,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 29,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 30,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 34,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 37,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 41,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 45,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 49,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 55,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 62,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 71,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 72,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 81,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 85,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 98,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 115,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 128,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 138,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 140,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 144,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 158,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 169,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 194,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 197,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 205,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 207,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 217,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 219,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 227,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 230,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 232,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 234,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 237,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 240,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 244,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 247,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 248,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 249,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                            (
                                                2,
                                                2,
                                            ),
                                            (
                                                3,
                                                3,
                                            ),
                                            (
                                                4,
                                                4,
                                            ),
                                            (
                                                5,
                                                5,
                                            ),
                                            (
                                                6,
                                                6,
                                            ),
                                            (
                                                7,
                                                7,
                                            ),
                                            (
                                                8,
                                                8,
                                            ),
                                            (
                                                9,
                                                9,
                                            ),
                                            (
                                                10,
                                                10,
                                            ),
                                            (
                                                11,
                                                11,
                                            ),
                                            (
                                                12,
                                                12,
                                            ),
                                            (
                                                13,
                                                13,
                                            ),
                                            (
                                                14,
                                                14,
                                            ),
                                            (
                                                15,
                                                15,
                                            ),
                                            (
                                                16,
                                                16,
                                            ),
                                            (
                                                17,
                                                17,
                                            ),
                                            (
                                                18,
                                                18,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        ItemSynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::FunctionFn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                        decl: FunctionFnFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                            template_parameters: [],
                            parenate_parameters: [
                                ParenateSynParameterData::Ordinary {
                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                    ty: 2,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 4,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                opd: 1,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
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
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `cc`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                9,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::MajorItem(
                                                            MajorItemSynNodePath::Fugitive(
                                                                FugitiveSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
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
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Tilde,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                opd: 1,
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Option,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                                opd: 3,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `f32`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            11,
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
                                                                    symbol_modifier_tokens: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `cc`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
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
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_syn_symbol_arena: Arena {
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                    access_end: None,
                                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                                OrdinaryParenateParameter {
                                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                                        syn_pattern_expr_idx: 1,
                                                                    },
                                                                    ty_expr_idx: 2,
                                                                },
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    syn_pattern_expr_roots: [
                                                        SynPatternExprRoot {
                                                            kind: SynPatternExprRootKind::Parenate,
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                    ],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ExplicitParameterType,
                                                            syn_expr_idx: 2,
                                                        },
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 4,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [
                                                        (
                                                            1,
                                                            1,
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::one::upmost`, `FunctionFn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::InheritedSynSymbol {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    inherited_syn_symbol_idx: 1,
                                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 59,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ropd: 5,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    condition: 6,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 8,
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
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
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
                                                        `dp`,
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
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
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
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    18,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `dp`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 9,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        ItemSynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::FunctionFn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                        decl: FunctionFnFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                            template_parameters: [],
                            parenate_parameters: [
                                ParenateSynParameterData::Ordinary {
                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                    ty: 2,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 4,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                opd: 1,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
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
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `cc`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                11,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::MajorItem(
                                                            MajorItemSynNodePath::Fugitive(
                                                                FugitiveSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
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
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Tilde,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                opd: 1,
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Option,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                                opd: 3,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `f32`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            11,
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
                                                                    symbol_modifier_tokens: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `cc`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
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
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_syn_symbol_arena: Arena {
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                    access_end: None,
                                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                                OrdinaryParenateParameter {
                                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                                        syn_pattern_expr_idx: 1,
                                                                    },
                                                                    ty_expr_idx: 2,
                                                                },
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    syn_pattern_expr_roots: [
                                                        SynPatternExprRoot {
                                                            kind: SynPatternExprRootKind::Parenate,
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                    ],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ExplicitParameterType,
                                                            syn_expr_idx: 2,
                                                        },
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 4,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [
                                                        (
                                                            1,
                                                            1,
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::one::downmost`, `FunctionFn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::InheritedSynSymbol {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    inherited_syn_symbol_idx: 1,
                                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 60,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ropd: 5,
                                                },
                                                SynExprData::InheritedSynSymbol {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    inherited_syn_symbol_idx: 1,
                                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    opd: 9,
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    condition: 6,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 10,
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
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
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
                                                        `dp`,
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
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
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
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    23,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `dp`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 10,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 11,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        ItemSynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::FunctionFn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                        decl: FunctionFnFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                            template_parameters: [],
                            parenate_parameters: [
                                ParenateSynParameterData::Ordinary {
                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                    ty: 2,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 4,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                opd: 1,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
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
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `cc`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                17,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::MajorItem(
                                                            MajorItemSynNodePath::Fugitive(
                                                                FugitiveSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
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
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Tilde,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                opd: 1,
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::num::f32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::Prefix {
                                                                opr: Option,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                                opd: 3,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `f32`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            11,
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
                                                                    symbol_modifier_tokens: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `cc`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
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
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_syn_symbol_arena: Arena {
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                    access_end: None,
                                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                                OrdinaryParenateParameter {
                                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                                        syn_pattern_expr_idx: 1,
                                                                    },
                                                                    ty_expr_idx: 2,
                                                                },
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    syn_pattern_expr_roots: [
                                                        SynPatternExprRoot {
                                                            kind: SynPatternExprRootKind::Parenate,
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                    ],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ExplicitParameterType,
                                                            syn_expr_idx: 2,
                                                        },
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 4,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [
                                                        (
                                                            1,
                                                            1,
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::one::hat`, `FunctionFn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::InheritedSynSymbol {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    inherited_syn_symbol_idx: 1,
                                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 61,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ropd: 5,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    LiteralData::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 62,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::Binary {
                                                    lopd: 8,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    ropd: 9,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 11,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    current_syn_symbol_idx: 1,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExprData::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    opd: 12,
                                                },
                                                SynExprData::Field {
                                                    owner: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        27,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Binary {
                                                    lopd: 14,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    ropd: 15,
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..5,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    condition: 6,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    condition: 10,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 16,
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
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
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
                                                        `dp`,
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
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
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
                                            current_syn_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    29,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `dp`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        syn_pattern_expr_roots: [
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 1,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 10,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 16,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 17,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [
                                            (
                                                1,
                                                1,
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ],
)