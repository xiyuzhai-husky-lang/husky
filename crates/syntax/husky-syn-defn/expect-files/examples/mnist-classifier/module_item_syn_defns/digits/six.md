[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 6,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
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
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
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
                                                FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        7,
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
                                            syn_expr_idx: 4,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
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
                                            ident: `upmost`,
                                            regional_token_idx: RegionalTokenIdx(
                                                6,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
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
                                syn_expr_idx: 5,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 6,
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
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 7,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
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
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
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
                                                FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        9,
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
                                            syn_expr_idx: 5,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        10,
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
                                            ident: `upmost`,
                                            regional_token_idx: RegionalTokenIdx(
                                                6,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `bottom1`,
                                            regional_token_idx: RegionalTokenIdx(
                                                8,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
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
                                syn_expr_idx: 6,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 7,
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
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 124,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
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
                                                        TypeVariantPath(
                                                            ItemPathId {
                                                                data: ItemPathData::TypeVariant(
                                                                    TypeVariantPathData {
                                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                        ident: `Six`,
                                                                        index: U8(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                        ),
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
                                                            8,
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
                                                            9,
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
                                                            10,
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
                                                        11,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `Six`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath(
                                                            ItemPathId {
                                                                data: ItemPathData::TypeVariant(
                                                                    TypeVariantPathData {
                                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                        ident: `Six`,
                                                                        index: U8(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                        ),
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
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
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
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 2,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 3,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `upmost_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Be {
                                    src: 5,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
                                            },
                                            variables: ArenaIdxRange(
                                                2..2,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 7,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `eff_holes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `lower_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 10,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `upper_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            32,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 11,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    ropd: 12,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `eff_holes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 14,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            36,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        38,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 15,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 16,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                                SynExprData::Be {
                                    src: 17,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 6,
                                            },
                                            variables: ArenaIdxRange(
                                                4..4,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 20,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            47,
                                        ),
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 9,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 22,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `rel_norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            51,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        55,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            5,
                                        ),
                                    ),
                                ),
                                SynExprData::FunctionCall {
                                    function: 19,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                    items: [
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 21,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        48,
                                                    ),
                                                ),
                                            },
                                        ),
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 23,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        52,
                                                    ),
                                                ),
                                            },
                                        ),
                                        Keyed(
                                            SynKeyedCallListItem {
                                                key_regional_token_idx: RegionalTokenIdx(
                                                    53,
                                                ),
                                                key: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 442,
                                                        },
                                                    ),
                                                ),
                                                argument_expr_idx: 24,
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 25,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 10,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 27,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            63,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        65,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 28,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        64,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 29,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `bottom1_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        70,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 31,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 32,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            73,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        75,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `bottom1_match_dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 34,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            81,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `upmost_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 36,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        86,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 37,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            88,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        90,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 38,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            92,
                                        ),
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 11,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 40,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        97,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            98,
                                        ),
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 12,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 42,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            102,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        104,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 63,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 43,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        103,
                                    ),
                                    ropd: 44,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `bottom1_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        106,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Be {
                                    src: 46,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 13,
                                            },
                                            variables: ArenaIdxRange(
                                                9..9,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        117,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 64,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `bottom1_match_dp_y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        114,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    opd: 48,
                                },
                                SynExprData::Binary {
                                    lopd: 49,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                    ropd: 50,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 14,
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
                                        121,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 65,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 52,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        120,
                                    ),
                                    ropd: 53,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 15,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `upmost_match_dp_y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        124,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        128,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            15,
                                        ),
                                    ),
                                ),
                                SynExprData::FunctionCall {
                                    function: 55,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    items: [
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 56,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        125,
                                                    ),
                                                ),
                                            },
                                        ),
                                        Keyed(
                                            SynKeyedCallListItem {
                                                key_regional_token_idx: RegionalTokenIdx(
                                                    126,
                                                ),
                                                key: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 442,
                                                        },
                                                    ),
                                                ),
                                                argument_expr_idx: 57,
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        129,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 58,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        130,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 17,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 18,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 61,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        139,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            140,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `upmost_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        144,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 63,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        145,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 64,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        146,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            147,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        148,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        149,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 62,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        141,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_point`,
                                        regional_token_idx: RegionalTokenIdx(
                                            142,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 65,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        150,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 19,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `upmost_match_dp_y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        153,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 20,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `lower_excess`,
                                    regional_token_idx: RegionalTokenIdx(
                                        157,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
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
                                        163,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            6,
                                        ),
                                    ),
                                ),
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 71,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        160,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `top_k_row_span_sum`,
                                        regional_token_idx: RegionalTokenIdx(
                                            161,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        162,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 72,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        164,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        168,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            15,
                                        ),
                                    ),
                                ),
                                SynExprData::FunctionCall {
                                    function: 67,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                    items: [
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 68,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        154,
                                                    ),
                                                ),
                                            },
                                        ),
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 69,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        156,
                                                    ),
                                                ),
                                            },
                                        ),
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 70,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        158,
                                                    ),
                                                ),
                                            },
                                        ),
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 73,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        165,
                                                    ),
                                                ),
                                            },
                                        ),
                                        Keyed(
                                            SynKeyedCallListItem {
                                                key_regional_token_idx: RegionalTokenIdx(
                                                    166,
                                                ),
                                                key: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 442,
                                                        },
                                                    ),
                                                ),
                                                argument_expr_idx: 74,
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        169,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 75,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        170,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `rel_upmost_match_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        172,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 11,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 77,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        173,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            174,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        176,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 66,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 78,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        175,
                                    ),
                                    ropd: 79,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 23,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 25,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
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
                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 84,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        189,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            190,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        194,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            5,
                                        ),
                                    ),
                                ),
                                SynExprData::FunctionCall {
                                    function: 83,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        187,
                                    ),
                                    items: [
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 85,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        191,
                                                    ),
                                                ),
                                            },
                                        ),
                                        Keyed(
                                            SynKeyedCallListItem {
                                                key_regional_token_idx: RegionalTokenIdx(
                                                    192,
                                                ),
                                                key: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 442,
                                                        },
                                                    ),
                                                ),
                                                argument_expr_idx: 86,
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        195,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 87,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        196,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 28,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 89,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        199,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            200,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        202,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 67,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 90,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        201,
                                    ),
                                    ropd: 91,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 29,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 93,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        206,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            207,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        209,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 68,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 94,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        208,
                                    ),
                                    ropd: 95,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `eff_holes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        211,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 97,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        212,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            213,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        215,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 98,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        214,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 99,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        216,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 100,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        217,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 101,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        218,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            219,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 102,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        220,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            221,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        222,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        223,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        225,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 69,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 103,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        224,
                                    ),
                                    ropd: 104,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `eff_holes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        227,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 106,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        228,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            229,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        231,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 107,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        230,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 108,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        232,
                                    ),
                                },
                                SynExprData::Be {
                                    src: 109,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        233,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 16,
                                            },
                                            variables: ArenaIdxRange(
                                                10..10,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `eff_holes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        240,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 111,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        241,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            242,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        244,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 112,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        243,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 113,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        245,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 114,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        246,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 115,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        247,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            248,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 116,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        249,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            250,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        251,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        252,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        254,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 117,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        253,
                                    ),
                                    ropd: 118,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `lower_excess`,
                                    regional_token_idx: RegionalTokenIdx(
                                        256,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        258,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 71,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 120,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        257,
                                    ),
                                    ropd: 121,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 32,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        20..31,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `six_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                4,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `major_connected_component`,
                                            regional_token_idx: RegionalTokenIdx(
                                                20,
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
                                                26,
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
                                                30,
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
                                            ident: `None`,
                                            regional_token_idx: RegionalTokenIdx(
                                                41,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `None`,
                                                        index: U8(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `narrow_down`,
                                            regional_token_idx: RegionalTokenIdx(
                                                43,
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
                                            ident: `six_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                45,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `six_match_refined1`,
                                            regional_token_idx: RegionalTokenIdx(
                                                49,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `six_match_refined1`,
                                            regional_token_idx: RegionalTokenIdx(
                                                61,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `six_match_refined1`,
                                            regional_token_idx: RegionalTokenIdx(
                                                96,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `six_match_refined1`,
                                            regional_token_idx: RegionalTokenIdx(
                                                100,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                108,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                            regional_token_idx: RegionalTokenIdx(
                                                119,
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
                                            ident: `narrow_down`,
                                            regional_token_idx: RegionalTokenIdx(
                                                122,
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
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                132,
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
                                    parent: 16,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            133,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                134,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `major_line_segment_sketch`,
                                            regional_token_idx: RegionalTokenIdx(
                                                138,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `narrow_down`,
                                            regional_token_idx: RegionalTokenIdx(
                                                151,
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
                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                            regional_token_idx: RegionalTokenIdx(
                                                155,
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
                                            ident: `major_connected_component`,
                                            regional_token_idx: RegionalTokenIdx(
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
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                179,
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
                                    parent: 22,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            180,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                181,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                183,
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
                                    parent: 24,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            184,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                185,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `narrow_down`,
                                            regional_token_idx: RegionalTokenIdx(
                                                186,
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
                                            ident: `six_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                188,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `six_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                198,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `six_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                205,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                234,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                259,
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
                                    parent: 31,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            260,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                261,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId {
                                                    data: ItemPathData::TypeVariant(
                                                        TypeVariantPathData {
                                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ident: `Yes`,
                                                            index: U8(
                                                                0,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
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
                                            113,
                                        ),
                                    },
                                    condition: 51,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            118,
                                        ),
                                    },
                                    condition: 54,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 59,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            131,
                                        ),
                                    },
                                    result: 60,
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            178,
                                        ),
                                    },
                                    result: 81,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 26,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            58,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 7,
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
                                                60,
                                            ),
                                        ),
                                    ),
                                    initial_value: 30,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            67,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 8,
                                            },
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
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                69,
                                            ),
                                        ),
                                    ),
                                    initial_value: 33,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            76,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 9,
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
                                                78,
                                            ),
                                        ),
                                    ),
                                    initial_value: 35,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            82,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 10,
                                            },
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
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                84,
                                            ),
                                        ),
                                    ),
                                    initial_value: 39,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            93,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 11,
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
                                                95,
                                            ),
                                        ),
                                    ),
                                    initial_value: 41,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            99,
                                        ),
                                    },
                                    condition: 45,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                105,
                                            ),
                                        },
                                        condition: Ok(
                                            47,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    112,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            1..5,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            135,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 14,
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
                                                137,
                                            ),
                                        ),
                                    ),
                                    initial_value: 66,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 76,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                171,
                                            ),
                                        },
                                        condition: Ok(
                                            80,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    177,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            5..6,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            182,
                                        ),
                                    },
                                    result: 82,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            204,
                                        ),
                                    },
                                    condition: 96,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            239,
                                        ),
                                    },
                                    condition: 119,
                                },
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
                                    initial_value: 4,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                    },
                                    condition: 6,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 4,
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
                                                19,
                                            ),
                                        ),
                                    ),
                                    initial_value: 8,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 5,
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
                                                25,
                                            ),
                                        ),
                                    ),
                                    initial_value: 13,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                33,
                                            ),
                                        },
                                        condition: Ok(
                                            18,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    42,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            6..18,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 88,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                197,
                                            ),
                                        },
                                        condition: Ok(
                                            92,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    203,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            18..19,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            210,
                                        ),
                                    },
                                    condition: 105,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                226,
                                            ),
                                        },
                                        condition: Ok(
                                            110,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    238,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            19..20,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            255,
                                        ),
                                    },
                                    condition: 122,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 123,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternExprRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `upmost_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 2,
                                        path: TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        lpar: LparRegionalToken(
                                            RegionalTokenIdx(
                                                14,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    2,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                16,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `eff_holes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `lower_excess`,
                                            regional_token_idx: RegionalTokenIdx(
                                                24,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::UnitTypeVariant {
                                        path_expr_idx: 6,
                                        path: TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `None`,
                                                        index: U8(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `bottom1_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                59,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `bottom1_match_dp`,
                                            regional_token_idx: RegionalTokenIdx(
                                                68,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `bottom1_match_dp_y`,
                                            regional_token_idx: RegionalTokenIdx(
                                                77,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `upmost_match_dp_y`,
                                            regional_token_idx: RegionalTokenIdx(
                                                83,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `others`,
                                            regional_token_idx: RegionalTokenIdx(
                                                94,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                110,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 13,
                                        path: TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        lpar: LparRegionalToken(
                                            RegionalTokenIdx(
                                                109,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    12,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                111,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `rel_upmost_match_end`,
                                            regional_token_idx: RegionalTokenIdx(
                                                136,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                236,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 30,
                                        path: TypeVariantPath(
                                            ItemPathId {
                                                data: ItemPathData::TypeVariant(
                                                    TypeVariantPathData {
                                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                        ident: `Some`,
                                                        index: U8(
                                                            0,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        lpar: LparRegionalToken(
                                            RegionalTokenIdx(
                                                235,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    15,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                237,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
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
                                        4,
                                    ),
                                    SynPatternSymbol::Atom(
                                        5,
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
                                        14,
                                    ),
                                    SynPatternSymbol::Atom(
                                        15,
                                    ),
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `upmost_match`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `_`,
                                        2,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `eff_holes`,
                                        3,
                                    ),
                                ],
                                [
                                    (
                                        `lower_excess`,
                                        4,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `bottom1_match`,
                                        5,
                                    ),
                                ],
                                [
                                    (
                                        `bottom1_match_dp`,
                                        6,
                                    ),
                                ],
                                [
                                    (
                                        `bottom1_match_dp_y`,
                                        7,
                                    ),
                                ],
                                [
                                    (
                                        `upmost_match_dp_y`,
                                        8,
                                    ),
                                ],
                                [
                                    (
                                        `others`,
                                        9,
                                    ),
                                ],
                                [
                                    (
                                        `_`,
                                        10,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `rel_upmost_match_end`,
                                        11,
                                    ),
                                ],
                                [
                                    (
                                        `_`,
                                        12,
                                    ),
                                ],
                                [],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
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
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            3,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    262,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `upmost_match`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            19,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    262,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `eff_holes`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            25,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    262,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `lower_excess`,
                                            pattern_symbol_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            60,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `bottom1_match`,
                                            pattern_symbol_idx: 5,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            69,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `bottom1_match_dp`,
                                            pattern_symbol_idx: 6,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            78,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `bottom1_match_dp_y`,
                                            pattern_symbol_idx: 7,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            84,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `upmost_match_dp_y`,
                                            pattern_symbol_idx: 8,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            95,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `others`,
                                            pattern_symbol_idx: 9,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            137,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `rel_upmost_match_end`,
                                            pattern_symbol_idx: 11,
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
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 3,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 4,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 5,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 6,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
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
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 10,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 11,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 13,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 14,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 16,
                            },
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 4,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 6,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 8,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 13,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 26,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 30,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 33,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 35,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 39,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 41,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 45,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 51,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 54,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 59,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 60,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 66,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 76,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 81,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 82,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 88,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 96,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 105,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 119,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 122,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 123,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 124,
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
                                3,
                                2,
                            ),
                            (
                                4,
                                3,
                            ),
                            (
                                5,
                                4,
                            ),
                            (
                                6,
                                5,
                            ),
                            (
                                7,
                                6,
                            ),
                            (
                                8,
                                7,
                            ),
                            (
                                9,
                                8,
                            ),
                            (
                                11,
                                9,
                            ),
                        ],
                    },
                },
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 9,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
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
                                                SynPatternExprData::Ident {
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
                                                Pure,
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
                                                Pure,
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
                                                    modifier: Pure,
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
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        ),
                        syn_expr_arena: Arena {
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
                                                    value: 72,
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
                                    SynPatternExprData::Ident {
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
                                    Pure,
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
                                    Pure,
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
                                        modifier: Pure,
                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                            ident: `cc`,
                                        },
                                    },
                                ],
                            },
                            current_syn_symbol_arena: Arena {
                                data: [
                                    CurrentSynSymbol {
                                        modifier: Pure,
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
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 36,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
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
                                                SynPatternExprData::Ident {
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
                                                Pure,
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
                                                Pure,
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
                                                    modifier: Pure,
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
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        ),
                        syn_expr_arena: Arena {
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
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        15,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
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
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    opd: 4,
                                },
                                SynExprData::Binary {
                                    lopd: 5,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: 6,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 8,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            21,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 10,
                                    opr: Closed(
                                        Div,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ropd: 11,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    item: 12,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 13,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `abs`,
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        32,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 74,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 14,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ropd: 15,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 17,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            36,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 18,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            38,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        42,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 75,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 19,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    ropd: 20,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 22,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `line_segment_sketch`,
                                        regional_token_idx: RegionalTokenIdx(
                                            48,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 23,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            50,
                                        ),
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 25,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            56,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 24,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_point`,
                                        regional_token_idx: RegionalTokenIdx(
                                            52,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 26,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `relative_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 28,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            63,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        65,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 76,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 29,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        64,
                                    ),
                                    ropd: 30,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 32,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            69,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        70,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 33,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            73,
                                        ),
                                    },
                                },
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    opd: 34,
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        2..8,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
                                    condition: 16,
                                },
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
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                        condition: Ok(
                                            7,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
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
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            33,
                                        ),
                                    },
                                    condition: 21,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            43,
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
                                                45,
                                            ),
                                        ),
                                    ),
                                    initial_value: 27,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            60,
                                        ),
                                    },
                                    condition: 31,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 35,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternExprRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `dp`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `relative_end`,
                                            regional_token_idx: RegionalTokenIdx(
                                                44,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Pure,
                                    Pure,
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
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `dp`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `relative_end`,
                                        2,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Pure,
                                    Pure,
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
                                        modifier: Pure,
                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                            ident: `cc`,
                                        },
                                    },
                                ],
                            },
                            current_syn_symbol_arena: Arena {
                                data: [
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            3,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    74,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `dp`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            45,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    74,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `relative_end`,
                                            pattern_symbol_idx: 2,
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
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 16,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 21,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 27,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 31,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 35,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 36,
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
                        ],
                    },
                },
            },
        ),
    ),
]