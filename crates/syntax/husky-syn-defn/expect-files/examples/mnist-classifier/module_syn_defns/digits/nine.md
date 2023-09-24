Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
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
                                                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 3,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 1,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 2,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 4,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
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
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
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
                                                PrincipalEntityPathExpr::Root {
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
                                                            FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
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
                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
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
                                                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 3,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 1,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 2,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 4,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
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
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
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
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `big_cc`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
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
                        path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
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
                                                        path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
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
                                                            ident: `Nine`,
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
                                            PrincipalEntityPathExpr::Root {
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
                                            PrincipalEntityPathExpr::Root {
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
                                            PrincipalEntityPathExpr::Subitem {
                                                parent: 3,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `Nine`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Nine`,
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
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                87,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
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
                                                                            ident: `Nine`,
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                            PrincipalEntityPathExpr::Subitem {
                                                                parent: 3,
                                                                colon_colon_token: ColonColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        11,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentRegionalToken {
                                                                        ident: `Nine`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::TypeVariant(
                                                                        TypeVariantPath {
                                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                            ident: `Nine`,
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
                                                            path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 1,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                    target: Ok(
                                                        BePatternObelisk {
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
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    target: Ok(
                                                        BePatternObelisk {
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
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 5,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `eff_holes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 8,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 9,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 10,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    target: Ok(
                                                        BePatternObelisk {
                                                            pattern_expr: 4,
                                                            variables: ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 13,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 14,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `down_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        34,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 16,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    target: Ok(
                                                        BePatternObelisk {
                                                            pattern_expr: 6,
                                                            variables: ArenaIdxRange(
                                                                6..7,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `down_match`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 18,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        41,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 19,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            43,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        44,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        45,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 20,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            47,
                                                        ),
                                                    },
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
                                                    owner: 22,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        52,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 23,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        56,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            57,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 24,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    ropd: 25,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `higher_excess`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        59,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        61,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 109,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 27,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        60,
                                                    ),
                                                    ropd: 28,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        63,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 30,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        64,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            65,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        67,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 31,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        66,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 32,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        68,
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 33,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        69,
                                                    ),
                                                    target: Ok(
                                                        BePatternObelisk {
                                                            pattern_expr: 9,
                                                            variables: ArenaIdxRange(
                                                                9..10,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 7,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 35,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        74,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            75,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        76,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        77,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        79,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 36,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        78,
                                                    ),
                                                    ropd: 37,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 8,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 39,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        84,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            85,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        87,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 40,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        86,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 41,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        88,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `nine_match_refine_result`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        90,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 43,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        91,
                                                    ),
                                                    target: Ok(
                                                        BePatternObelisk {
                                                            pattern_expr: 11,
                                                            variables: ArenaIdxRange(
                                                                11..12,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 9,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 45,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        95,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        98,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 110,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 46,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        97,
                                                    ),
                                                    ropd: 47,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 10,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
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
                                                    owner: 49,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        103,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            104,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 50,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        107,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            108,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 51,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        105,
                                                    ),
                                                    ropd: 52,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 12,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 54,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        113,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            114,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        116,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 55,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        115,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 56,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        117,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `upper_arc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        119,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 58,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        120,
                                                    ),
                                                    target: Ok(
                                                        BePatternObelisk {
                                                            pattern_expr: 14,
                                                            variables: ArenaIdxRange(
                                                                14..15,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `upper_arc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        123,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 60,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        124,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 61,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        125,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `displacement`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            126,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        127,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        128,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 62,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        129,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            130,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        132,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 111,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 63,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        131,
                                                    ),
                                                    ropd: 64,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `upper_arc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        134,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 66,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        135,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        140,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 112,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 67,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        136,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            137,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        139,
                                                    ),
                                                    opd: 68,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 69,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        138,
                                                    ),
                                                    ropd: 70,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 13,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 72,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        143,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            144,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        146,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 113,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 73,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        145,
                                                    ),
                                                    ropd: 74,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 14,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        154,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 76,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        151,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `top_k_row_right_mass_sum`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            152,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        153,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 77,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        155,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `a`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        157,
                                                    ),
                                                    current_symbol_idx: 15,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 15,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        159,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 114,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 79,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        158,
                                                    ),
                                                    ropd: 80,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `a`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        161,
                                                    ),
                                                    current_symbol_idx: 15,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 15,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        163,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 115,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 82,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        162,
                                                    ),
                                                    ropd: 83,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 16,
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
                                                    item_path_expr: 18,
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
                                                        15..26,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_zero`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_six`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `nine_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                27,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                51,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                55,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_concave_components`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                73,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `nine_match_refine`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                83,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `nine_match_refine`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                94,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                102,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                106,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `nine_match_refine`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                112,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `nine_match_refine`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                142,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                150,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                165,
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
                                                    parent: 15,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            166,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                167,
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
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                168,
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
                                                    parent: 17,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            169,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                170,
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
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            72,
                                                        ),
                                                    },
                                                    condition: 38,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            80,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                82,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 42,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            89,
                                                        ),
                                                    },
                                                    condition: 44,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            93,
                                                        ),
                                                    },
                                                    condition: 48,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            99,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                101,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 53,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            109,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                111,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 57,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            118,
                                                        ),
                                                    },
                                                    condition: 59,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            122,
                                                        ),
                                                    },
                                                    condition: 65,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            133,
                                                        ),
                                                    },
                                                    condition: 71,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            141,
                                                        ),
                                                    },
                                                    condition: 75,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            147,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                149,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 78,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            156,
                                                        ),
                                                    },
                                                    condition: 81,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            160,
                                                        ),
                                                    },
                                                    condition: 84,
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                    result: 85,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    condition: 2,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    condition: 4,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                11,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 6,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    condition: 11,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                26,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 15,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            33,
                                                        ),
                                                    },
                                                    condition: 17,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            37,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                39,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 21,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            48,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                50,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 26,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                    condition: 29,
                                                },
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                62,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            34,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        71,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            1..15,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 86,
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
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `eff_holes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                23,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `down_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                25,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                36,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `down_match_dp_y`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                38,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `higher_excess`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                49,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                70,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `nine_match_refine_result`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                81,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                92,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `higher_excess`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                100,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `upper_arc`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                110,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                121,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `a`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                148,
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
                                                        `eff_holes`,
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
                                                        `down_match`,
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
                                                        `down_match_dp_y`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `higher_excess`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `none`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `nine_match_refine_result`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `higher_excess`,
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `upper_arc`,
                                                        13,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        14,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        15,
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
                                                        access_start: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `eff_holes`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `none`,
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
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `down_match`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            37,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            39,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `down_match_dp_y`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            50,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    171,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `higher_excess`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            71,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    168,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            82,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    168,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `nine_match_refine_result`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            93,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    168,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 11,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            101,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    168,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `higher_excess`,
                                                            pattern_symbol_idx: 12,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            111,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    168,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `upper_arc`,
                                                            pattern_symbol_idx: 13,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            122,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    168,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 14,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            149,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    168,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 15,
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
                                                kind: LetStmtInitialValue,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 11,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 15,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 17,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 21,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 26,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 29,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 38,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 42,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 44,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 48,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 53,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 57,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 59,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 65,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 71,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 75,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 78,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 81,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 84,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 85,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 86,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 87,
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
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 1,
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
                                ReturnTypeBeforeColonObelisk {
                                    expr: 4,
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
                                                        path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
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
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    6,
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
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
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
                                            PrincipalEntityPathExpr::Root {
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
                                                    symbol_modifier_keyword_group: None,
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
                                                    access_start: RegionalTokenIdx(
                                                        5,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
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
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    6,
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
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                                opd: 3,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    symbol_modifier_keyword_group: None,
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
                                                                    access_start: RegionalTokenIdx(
                                                                        5,
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
                                                            path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
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
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
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
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
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
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 116,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ropd: 5,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
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
                                                SynExpr::Block {
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
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    condition: 6,
                                                },
                                                SynStmt::Eval {
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
                                                        symbol_modifier_keyword_group: None,
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
                                            pattern_infos: [
                                                Let,
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
                                                        variant: CurrentSynSymbolVariant::LetVariable {
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
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 9,
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
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 1,
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
                                ReturnTypeBeforeColonObelisk {
                                    expr: 4,
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
                                                        path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
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
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    6,
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
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
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
                                            PrincipalEntityPathExpr::Root {
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
                                                    symbol_modifier_keyword_group: None,
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
                                                    access_start: RegionalTokenIdx(
                                                        5,
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
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                15,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
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
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    6,
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
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                                opd: 3,
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    symbol_modifier_keyword_group: None,
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
                                                                    access_start: RegionalTokenIdx(
                                                                        5,
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
                                                            path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
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
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
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
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `dp`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
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
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 117,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ropd: 5,
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymin`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 118,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 9,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    ropd: 10,
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymin`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                },
                                                SynExpr::Block {
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
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    condition: 6,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    condition: 11,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 14,
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
                                            pattern_infos: [
                                                Let,
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
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    32,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
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
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 11,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 14,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 15,
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