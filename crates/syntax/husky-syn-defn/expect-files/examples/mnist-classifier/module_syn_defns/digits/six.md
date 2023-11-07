Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        decl: ValFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 1,
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                    symbol_region: SynSymbolRegion {
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
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                6,
                                SynExprRegion {
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
                                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                    symbol_region: SynSymbolRegion {
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
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                        symbol_region: SynSymbolRegion {
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
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        decl: ValFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 1,
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                    symbol_region: SynSymbolRegion {
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
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                7,
                                SynExprRegion {
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
                                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                    symbol_region: SynSymbolRegion {
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
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                        symbol_region: SynSymbolRegion {
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
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                        decl: ValFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 5,
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                            ident: `Six`,
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
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Six`,
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
                                    symbol_region: SynSymbolRegion {
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
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                126,
                                SynExprRegion {
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
                                                                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                                            ident: `Six`,
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
                                                                        TypeVariantPath {
                                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                            ident: `Six`,
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
                                                    symbol_region: SynSymbolRegion {
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
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Be {
                                                    src: 1,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 1,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 4,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 5,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Be {
                                                    src: 7,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 3,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                3..4,
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
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `eff_holes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
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
                                                    owner: 11,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            33,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Binary {
                                                    lopd: 13,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    ropd: 14,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    current_syn_symbol_idx: 4,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 16,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        36,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            37,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 17,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        38,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 18,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                },
                                                SynExprData::Be {
                                                    src: 19,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        41,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 6,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                6..7,
                                                            ),
                                                        },
                                                    ),
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
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 22,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        47,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            48,
                                                        ),
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 8,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 24,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        51,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `rel_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            52,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        56,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 21,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        45,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 23,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        49,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 25,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        53,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    54,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 26,
                                                                separator: None,
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        57,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 27,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        58,
                                                    ),
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
                                                    owner: 29,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        63,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            64,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        66,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 30,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        65,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 31,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        67,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `bottom1_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        71,
                                                    ),
                                                    current_syn_symbol_idx: 7,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 33,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        72,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 34,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        73,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            74,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        75,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        76,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `bottom1_match_dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        80,
                                                    ),
                                                    current_syn_symbol_idx: 8,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 36,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        81,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            82,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        86,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 38,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        87,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 39,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        88,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            89,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        90,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        91,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 40,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        92,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            93,
                                                        ),
                                                    },
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
                                                    owner: 42,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        98,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            99,
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
                                                    owner: 44,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        102,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            103,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        105,
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
                                                    lopd: 45,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        104,
                                                    ),
                                                    ropd: 46,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `bottom1_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        107,
                                                    ),
                                                    current_syn_symbol_idx: 7,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                SynExprData::Be {
                                                    src: 48,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        108,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 12,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                12..13,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        115,
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
                                                        112,
                                                    ),
                                                    current_syn_symbol_idx: 9,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                SynExprData::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        114,
                                                    ),
                                                    opd: 50,
                                                },
                                                SynExprData::Binary {
                                                    lopd: 51,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        113,
                                                    ),
                                                    ropd: 52,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 12,
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
                                                        119,
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
                                                    lopd: 54,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        118,
                                                    ),
                                                    ropd: 55,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 13,
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
                                                        122,
                                                    ),
                                                    current_syn_symbol_idx: 10,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        126,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            15,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 57,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        121,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 58,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        123,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    124,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 59,
                                                                separator: None,
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        127,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 60,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        128,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 15,
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
                                                    path_expr_idx: 16,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 63,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        137,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `upmost_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        142,
                                                    ),
                                                    current_syn_symbol_idx: 2,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExprData::Suffix {
                                                    opd: 65,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        143,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 66,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        144,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            145,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        146,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        147,
                                                    ),
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 64,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        139,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_point`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            140,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        141,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 67,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        148,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 17,
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
                                                        151,
                                                    ),
                                                    current_syn_symbol_idx: 10,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 18,
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
                                                        155,
                                                    ),
                                                    current_syn_symbol_idx: 5,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 19,
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
                                                        161,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            6,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 73,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        158,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `top_k_row_span_sum`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            159,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        160,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 74,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        162,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        166,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            15,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 69,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        150,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 70,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        152,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 71,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        154,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 72,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        156,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 75,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        163,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    164,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 76,
                                                                separator: None,
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        167,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 77,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        168,
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `rel_upmost_match_end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        170,
                                                    ),
                                                    current_syn_symbol_idx: 13,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 79,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        171,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            172,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        174,
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
                                                    lopd: 80,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        173,
                                                    ),
                                                    ropd: 81,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 21,
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
                                                    path_expr_idx: 23,
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
                                                    path_expr_idx: 24,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 25,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 86,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        187,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            188,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        192,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::FunctionCall {
                                                    function: 85,
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        185,
                                                    ),
                                                    items: [
                                                        RegularOrVariadic(
                                                            SynRegularOrVariadicCallListItem {
                                                                argument_expr_idx: 87,
                                                                separator: Comma(
                                                                    RegionalTokenIdx(
                                                                        189,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Keyed(
                                                            SynKeyedCallListItem {
                                                                key_regional_token_idx: RegionalTokenIdx(
                                                                    190,
                                                                ),
                                                                key: Ident(
                                                                    Coword(
                                                                        Id {
                                                                            value: 453,
                                                                        },
                                                                    ),
                                                                ),
                                                                argument_expr_idx: 88,
                                                                separator: None,
                                                            },
                                                        ),
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        193,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 89,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        194,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 26,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 91,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        197,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            198,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        200,
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
                                                    lopd: 92,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        199,
                                                    ),
                                                    ropd: 93,
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
                                                    owner: 95,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        204,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            205,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        207,
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
                                                    lopd: 96,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        206,
                                                    ),
                                                    ropd: 97,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        209,
                                                    ),
                                                    current_syn_symbol_idx: 4,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 99,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        210,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            211,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        213,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 100,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        212,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 101,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        214,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 102,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        215,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 103,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        216,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            217,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 104,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        218,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymax`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            219,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        220,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        221,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        223,
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
                                                    lopd: 105,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        222,
                                                    ),
                                                    ropd: 106,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        225,
                                                    ),
                                                    current_syn_symbol_idx: 4,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 108,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        226,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            227,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        229,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 109,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        228,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 110,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        230,
                                                    ),
                                                },
                                                SynExprData::Be {
                                                    src: 111,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        231,
                                                    ),
                                                    target: Ok(
                                                        BePatternSynSyndicate {
                                                            pattern_expr_root: BeSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 14,
                                                            },
                                                            variables: ArenaIdxRange(
                                                                14..15,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        235,
                                                    ),
                                                    current_syn_symbol_idx: 4,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                SynExprData::Field {
                                                    owner: 113,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        236,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            237,
                                                        ),
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        239,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::IndexOrCompositionWithList {
                                                    owner: 114,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        238,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 115,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        240,
                                                    ),
                                                },
                                                SynExprData::Suffix {
                                                    opd: 116,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        241,
                                                    ),
                                                },
                                                SynExprData::Field {
                                                    owner: 117,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        242,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            243,
                                                        ),
                                                    },
                                                },
                                                SynExprData::MethodApplicationOrCall {
                                                    self_argument: 118,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        244,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymax`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            245,
                                                        ),
                                                    },
                                                    template_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        246,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        247,
                                                    ),
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        249,
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
                                                    lopd: 119,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        248,
                                                    ),
                                                    ropd: 120,
                                                },
                                                SynExprData::CurrentSynSymbol {
                                                    ident: `lower_excess`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        251,
                                                    ),
                                                    current_syn_symbol_idx: 5,
                                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        253,
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
                                                    lopd: 122,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        252,
                                                    ),
                                                    ropd: 123,
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 29,
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
                                                        20..32,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_one`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `six_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
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
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
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
                                                                27,
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
                                                                31,
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
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                44,
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
                                                                46,
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
                                                                50,
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
                                                                62,
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
                                                                97,
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
                                                                101,
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
                                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                117,
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
                                                                120,
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
                                                                130,
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
                                                    parent: 14,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            131,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                132,
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
                                                            ident: `major_line_segment_sketch`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                136,
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
                                                                149,
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
                                                                153,
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
                                                                157,
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
                                                                177,
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
                                                    parent: 20,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            178,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                179,
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
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                181,
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
                                                            182,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                183,
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
                                                            ident: `narrow_down`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                184,
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
                                                                186,
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
                                                                196,
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
                                                                203,
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
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                254,
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
                                                    parent: 28,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            255,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                256,
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
                                                            111,
                                                        ),
                                                    },
                                                    condition: 53,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            116,
                                                        ),
                                                    },
                                                    condition: 56,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 61,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            129,
                                                        ),
                                                    },
                                                    result: 62,
                                                },
                                                SynStmtData::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            176,
                                                        ),
                                                    },
                                                    result: 83,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 28,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            59,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 7,
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
                                                                61,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 32,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            68,
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
                                                                70,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 35,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            77,
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
                                                                79,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 37,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            83,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 10,
                                                            },
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
                                                        EqRegionalToken(
                                                            RegionalTokenIdx(
                                                                85,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 41,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            94,
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
                                                                96,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 43,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            100,
                                                        ),
                                                    },
                                                    condition: 47,
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                106,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            49,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    110,
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
                                                            133,
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
                                                                135,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 68,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 78,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                169,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            82,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    175,
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
                                                            180,
                                                        ),
                                                    },
                                                    result: 84,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            202,
                                                        ),
                                                    },
                                                    condition: 98,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            234,
                                                        ),
                                                    },
                                                    condition: 121,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    condition: 2,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
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
                                                                7,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 6,
                                                },
                                                SynStmtData::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                    condition: 8,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
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
                                                                20,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 10,
                                                },
                                                SynStmtData::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternSynSyndicate {
                                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                                syn_pattern_expr_idx: 5,
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
                                                                26,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 15,
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            20,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    43,
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
                                                    expr_idx: 90,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                195,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            94,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    201,
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
                                                            208,
                                                        ),
                                                    },
                                                    condition: 107,
                                                },
                                                SynStmtData::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                224,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            112,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    233,
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
                                                            250,
                                                        ),
                                                    },
                                                    condition: 124,
                                                },
                                                SynStmtData::Eval {
                                                    expr_idx: 125,
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
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `upmost_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `eff_holes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `lower_excess`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                25,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                42,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `bottom1_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                60,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `bottom1_match_dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                69,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `bottom1_match_dp_y`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                78,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `upmost_match_dp_y`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                84,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `others`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                95,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                109,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `rel_upmost_match_end`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                134,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                232,
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
                                                        `upmost_match`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `eff_holes`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `lower_excess`,
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
                                                        `bottom1_match`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `bottom1_match_dp`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `bottom1_match_dp_y`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `upmost_match_dp_y`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `others`,
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `rel_upmost_match_end`,
                                                        13,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        14,
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
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
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
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `upmost_match`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `eff_holes`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            26,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `lower_excess`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            43,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    184,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            61,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    184,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `bottom1_match`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            70,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    184,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `bottom1_match_dp`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            79,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    184,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `bottom1_match_dp_y`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            85,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    184,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `upmost_match_dp_y`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            96,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    184,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 11,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            110,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    133,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 12,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            135,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    184,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::LetVariable {
                                                            ident: `rel_upmost_match_end`,
                                                            pattern_symbol_idx: 13,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            233,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    250,
                                                                ),
                                                            ),
                                                        ),
                                                        data: CurrentSynSymbolData::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 14,
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
                                                kind: SynPatternExprRootKind::Be,
                                                syn_pattern_expr_idx: 1,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 2,
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
                                                syn_pattern_expr_idx: 12,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Let,
                                                syn_pattern_expr_idx: 13,
                                            },
                                            SynPatternExprRoot {
                                                kind: SynPatternExprRootKind::Be,
                                                syn_pattern_expr_idx: 14,
                                            },
                                        ],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 10,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 15,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 28,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 32,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 35,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 37,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 41,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 43,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 47,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 53,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 56,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 61,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 62,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::LetStmtInitialValue,
                                                syn_expr_idx: 68,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 78,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 83,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::ReturnExpr,
                                                syn_expr_idx: 84,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 90,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 98,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 107,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 121,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::Condition,
                                                syn_expr_idx: 124,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 125,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 126,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::FunctionFn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                        decl: FunctionFnFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
                            template_parameters: [],
                            parenate_parameters: [
                                ParenateParameterSyndicate::Ordinary {
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
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                    symbol_region: SynSymbolRegion {
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
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::MajorItem(
                                                            MajorItemSynNodePath::Fugitive(
                                                                FugitiveSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                                    symbol_region: SynSymbolRegion {
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
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                        symbol_region: SynSymbolRegion {
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
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::FunctionFn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
                        decl: FunctionFnFugitiveSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
                            template_parameters: [],
                            parenate_parameters: [
                                ParenateParameterSyndicate::Ordinary {
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
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                    symbol_region: SynSymbolRegion {
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
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                36,
                                SynExprRegion {
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
                                                                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                                    symbol_region: SynSymbolRegion {
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
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_tokens: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `dp`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
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
                                                    None,
                                                    None,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
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
                                                        modifier: None,
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