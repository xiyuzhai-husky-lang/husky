[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 4,
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
                                                                        path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                        ident: `ConnectedComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                                                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist::input`, `Val`),
                                            ),
                                        ),
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
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        4,
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
                                            ident: `find_connected_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `input`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist::input`, `Val`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 3,
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
                                syn_expr_idx: 3,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 4,
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
                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 23,
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
                                                                        path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                opd: 1,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConnectedComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            syn_expr_idx: 2,
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
                                                            path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 127,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        4,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 3,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                },
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
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        4,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 7,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 8,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `row_span_sum`,
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 11,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ropd: 12,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 14,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ropd: 15,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i0`,
                                    regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        4,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 17,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    ropd: 18,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i0`,
                                    regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 20,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 21,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        5..9,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `connected_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                14,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `connected_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                23,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `connected_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                41,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 16,
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
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
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
                                                22,
                                            ),
                                        ),
                                    ),
                                    initial_value: 10,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                29,
                                            ),
                                        },
                                        condition: Ok(
                                            13,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    33,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            1..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
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
                                                4,
                                            ),
                                        ),
                                    ),
                                    initial_value: 1,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
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
                                                9,
                                            ),
                                        ),
                                    ),
                                    initial_value: 2,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: 4,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: None,
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        5,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                step: Constant(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 3,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        3..5,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            40,
                                        ),
                                    },
                                    result: 22,
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternExprRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `i0`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `max_row_span_sum`,
                                            regional_token_idx: RegionalTokenIdx(
                                                8,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `row_span_sum`,
                                            regional_token_idx: RegionalTokenIdx(
                                                21,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Move,
                                    Move,
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
                                        3,
                                    ),
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `i0`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `max_row_span_sum`,
                                        2,
                                    ),
                                ],
                                [
                                    (
                                        `row_span_sum`,
                                        3,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
                                    Mut,
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
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            4,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    45,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `i0`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            9,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    45,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `max_row_span_sum`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            20,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    40,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i`,
                                            expr_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            22,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    40,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `row_span_sum`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        3..4,
                                    ),
                                ),
                            ],
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
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 10,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 16,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 19,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 22,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 23,
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
                                4,
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
                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 16,
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
                                                                        path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
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
                                                            path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
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
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 128,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        3,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 2,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 3,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ropd: 4,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        3,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 6,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 7,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 8,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `row_span_sum`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 9,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ropd: 10,
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
                                SynExprData::CurrentSynSymbol {
                                    ident: `sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        24,
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
                                        ident: `row_span_sum`,
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 13,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    ropd: 14,
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        2..5,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `connected_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `connected_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 11,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
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
                                                4,
                                            ),
                                        ),
                                    ),
                                    initial_value: 1,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: 3,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: None,
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        4,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                step: Constant(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 2,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    result: 15,
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternExprRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `sum`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Move,
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
                                        `sum`,
                                        1,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
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
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            4,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    29,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `sum`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            15,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    23,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i`,
                                            expr_idx: 3,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        2..3,
                                    ),
                                ),
                            ],
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
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 11,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 15,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 16,
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
                FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 3,
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
                                                                        path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
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
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    5,
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
                                                        ident: `RawContour`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                            syn_expr_idx: 4,
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
                                                            path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
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
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `raw_contours`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
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
                                            ident: `major_connected_component`,
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 3,
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
                FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 5,
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
                                                                        path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
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
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                opd: 1,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `RawContour`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                            syn_expr_idx: 2,
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
                                                            path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
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
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `raw_contours`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        5,
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
                                        4,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 3,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        6,
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
                                            ident: `major_connected_component`,
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 4,
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
                                syn_expr_idx: 4,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 5,
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
                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 3,
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
                                                                        path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                opd: 1,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `LineSegmentSketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                            syn_expr_idx: 2,
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
                                                            path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                                FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `line_segment_sketch`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
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
                                            ident: `major_raw_contour`,
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
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
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 3,
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
                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 3,
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
                                                                        path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
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
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    5,
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
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            syn_expr_idx: 4,
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
                                                            path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `concave_components`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
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
                                            ident: `major_line_segment_sketch`,
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 3,
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