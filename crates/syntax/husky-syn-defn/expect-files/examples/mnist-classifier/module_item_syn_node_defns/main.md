```rust
[
    (
        ItemSynNodePath::Submodule(
            Room32,
            SubmoduleSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Submodule(
                        SubmoduleSynNodePathData {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmoduleItemPath(
                                    ItemPathId(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::Submodule(
            Room32,
            SubmoduleSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Submodule(
                        SubmoduleSynNodePathData {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmoduleItemPath(
                                    ItemPathId(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::Submodule(
            Room32,
            SubmoduleSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Submodule(
                        SubmoduleSynNodePathData {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmoduleItemPath(
                                    ItemPathId(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::Submodule(
            Room32,
            SubmoduleSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Submodule(
                        SubmoduleSynNodePathData {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmoduleItemPath(
                                    ItemPathId(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::Submodule(
            Room32,
            SubmoduleSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Submodule(
                        SubmoduleSynNodePathData {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmoduleItemPath(
                                    ItemPathId(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::Submodule(
            Room32,
            SubmoduleSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Submodule(
                        SubmoduleSynNodePathData {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmoduleItemPath(
                                    ItemPathId(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::Submodule(
            Room32,
            SubmoduleSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Submodule(
                        SubmoduleSynNodePathData {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: SubmoduleItemPath(
                                    ItemPathId(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Fugitive(
                FugitiveSynNodePath(`mnist_classifier::main`, `Val`, (0)),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 20,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(`mnist_classifier::main`, `Val`, (0)),
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
                                    symbol_region: VariableRegionData {
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
                                            syn_expr_idx: 3,
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
                                    FugitiveSynNodePath(`mnist_classifier::main`, `Val`, (0)),
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
                                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 1,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 3,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 5,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 7,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 9,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 11,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 13,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 8,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 15,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 9,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 17,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 11,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 250,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        1..11,
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
                                                1,
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
                                            ident: `is_six`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `is_zero`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `is_seven`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `is_eight`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `is_three`,
                                            regional_token_idx: RegionalTokenIdx(
                                                11,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `is_nine`,
                                            regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `is_five`,
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `is_two`,
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `Class`,
                                            regional_token_idx: RegionalTokenIdx(
                                                19,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::Class`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 10,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            20,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Unknown`,
                                            regional_token_idx: RegionalTokenIdx(
                                                21,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 250,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 2,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 4,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 6,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 8,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 10,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 12,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 14,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 16,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 18,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 19,
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
                        symbol_region: VariableRegionData {
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
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 4,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 6,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 8,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 10,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 12,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 14,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 16,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 18,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 19,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 20,
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
]
```