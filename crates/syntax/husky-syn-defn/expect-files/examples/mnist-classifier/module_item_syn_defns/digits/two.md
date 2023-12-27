[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 8,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
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
                                            ident: `left_cc_pattern`,
                                            regional_token_idx: RegionalTokenIdx(
                                                6,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `right_cc_pattern`,
                                            regional_token_idx: RegionalTokenIdx(
                                                8,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `down_cc_pattern`,
                                            regional_token_idx: RegionalTokenIdx(
                                                10,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
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
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
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
                                                    value: 119,
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
                FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
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
                                                    value: 120,
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
                FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
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
                                        ident: `x`,
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
                                                    value: 121,
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
                                        ident: `x`,
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
                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
            ),
        ),
        Some(
            ItemSynDefn {
                body: 110,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                                        ident: `Two`,
                                                                        index: U8(
                                                                            2,
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
                                                        ident: `Two`,
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
                                                                        ident: `Two`,
                                                                        index: U8(
                                                                            2,
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
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
                                    owner: 3,
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
                                SynExprData::CurrentSynSymbol {
                                    ident: `eff_holes`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 5,
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
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        20,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 6,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 7,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                                SynExprData::Be {
                                    src: 8,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
                                            },
                                            variables: ArenaIdxRange(
                                                3..3,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 10,
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
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        31,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 11,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 12,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 14,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            38,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        40,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 15,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 16,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 18,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            47,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        49,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 19,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 20,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `cc_num`,
                                    regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        54,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            3,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 22,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                    ropd: 23,
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
                                    owner: 25,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `lower_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            60,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 26,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `upper_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            64,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 27,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    ropd: 28,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `lower_excess`,
                                    regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        68,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 122,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 30,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                    ropd: 31,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `cc_num`,
                                    regional_token_idx: RegionalTokenIdx(
                                        70,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        72,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 33,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                    ropd: 34,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        75,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Be {
                                    src: 36,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 9,
                                            },
                                            variables: ArenaIdxRange(
                                                7..7,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        82,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Be {
                                    src: 38,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 11,
                                            },
                                            variables: ArenaIdxRange(
                                                7..7,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 40,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 41,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        93,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            94,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        99,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 123,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        98,
                                    ),
                                    opd: 43,
                                },
                                SynExprData::Binary {
                                    lopd: 44,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        97,
                                    ),
                                    ropd: 45,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        103,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 47,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 48,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end_tangent`,
                                        regional_token_idx: RegionalTokenIdx(
                                            106,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        108,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        112,
                                    ),
                                    LiteralData::Bool(
                                        True,
                                    ),
                                ),
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 49,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle`,
                                        regional_token_idx: RegionalTokenIdx(
                                            110,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        111,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 50,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        117,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 52,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        118,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 53,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        119,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end_tangent`,
                                        regional_token_idx: RegionalTokenIdx(
                                            120,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        121,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        122,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 54,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            124,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        128,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 56,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        129,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 57,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        130,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end_tangent`,
                                        regional_token_idx: RegionalTokenIdx(
                                            131,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        132,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        133,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 58,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        134,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            135,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        139,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 60,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        140,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 61,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        141,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            142,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 62,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            144,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        145,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        146,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        150,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 64,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        151,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 65,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            153,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 66,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        154,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            155,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        156,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        157,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_ymax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        162,
                                    ),
                                    current_syn_symbol_idx: 11,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 13,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_ymin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        164,
                                    ),
                                    current_syn_symbol_idx: 12,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 14,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 68,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        163,
                                    ),
                                    ropd: 69,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        161,
                                    ),
                                    item: 70,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        165,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        167,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 124,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 71,
                                    opr: Closed(
                                        Div,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        166,
                                    ),
                                    ropd: 72,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        171,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 74,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        172,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 75,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        173,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            174,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 76,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        175,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            176,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        177,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        178,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        182,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 78,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        183,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 79,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        184,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            185,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 80,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        186,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            187,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        188,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        189,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_ymax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        194,
                                    ),
                                    current_syn_symbol_idx: 14,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 16,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_ymin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        196,
                                    ),
                                    current_syn_symbol_idx: 15,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 17,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 82,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        195,
                                    ),
                                    ropd: 83,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        193,
                                    ),
                                    item: 84,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        197,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        199,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 125,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 85,
                                    opr: Closed(
                                        Div,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        198,
                                    ),
                                    ropd: 86,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_mid_y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        201,
                                    ),
                                    current_syn_symbol_idx: 13,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 15,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_mid_y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        203,
                                    ),
                                    current_syn_symbol_idx: 16,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 18,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 88,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        202,
                                    ),
                                    ropd: 89,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `cc_num`,
                                    regional_token_idx: RegionalTokenIdx(
                                        205,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        207,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            3,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 91,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        206,
                                    ),
                                    ropd: 92,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `left_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        210,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Be {
                                    src: 94,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        211,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 23,
                                            },
                                            variables: ArenaIdxRange(
                                                17..17,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `right_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        217,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Be {
                                    src: 96,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        218,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 25,
                                            },
                                            variables: ArenaIdxRange(
                                                17..17,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `down_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        224,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Be {
                                    src: 98,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        225,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 27,
                                            },
                                            variables: ArenaIdxRange(
                                                17..17,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `down_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        231,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 100,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        232,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 101,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        233,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            234,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 102,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        235,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            236,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        237,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        238,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        240,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 126,
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
                                        239,
                                    ),
                                    ropd: 104,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `down_cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        244,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 106,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        245,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 107,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        246,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            247,
                                        ),
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 15,
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
                                            ident: `major_concave_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                4,
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
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `None`,
                                            regional_token_idx: RegionalTokenIdx(
                                                23,
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
                                            ident: `two_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                27,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `two_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                36,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `two_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                45,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `major_connected_component`,
                                            regional_token_idx: RegionalTokenIdx(
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
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `major_connected_component`,
                                            regional_token_idx: RegionalTokenIdx(
                                                62,
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
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                77,
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
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                84,
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
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                212,
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
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                219,
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
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                226,
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
                                                248,
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
                                            249,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                250,
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
                                            74,
                                        ),
                                    },
                                    condition: 37,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            81,
                                        ),
                                    },
                                    condition: 39,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            88,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 12,
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
                                                90,
                                            ),
                                        ),
                                    ),
                                    initial_value: 42,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            95,
                                        ),
                                    },
                                    condition: 46,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            100,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 13,
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
                                                102,
                                            ),
                                        ),
                                    ),
                                    initial_value: 51,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            114,
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
                                                116,
                                            ),
                                        ),
                                    ),
                                    initial_value: 55,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            125,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 15,
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
                                                127,
                                            ),
                                        ),
                                    ),
                                    initial_value: 59,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            136,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 16,
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
                                                138,
                                            ),
                                        ),
                                    ),
                                    initial_value: 63,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            147,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 17,
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
                                                149,
                                            ),
                                        ),
                                    ),
                                    initial_value: 67,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            158,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 18,
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
                                                160,
                                            ),
                                        ),
                                    ),
                                    initial_value: 73,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            168,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 19,
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
                                                170,
                                            ),
                                        ),
                                    ),
                                    initial_value: 77,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            179,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 20,
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
                                                181,
                                            ),
                                        ),
                                    ),
                                    initial_value: 81,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            190,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 21,
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
                                                192,
                                            ),
                                        ),
                                    ),
                                    initial_value: 87,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            200,
                                        ),
                                    },
                                    condition: 90,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            209,
                                        ),
                                    },
                                    condition: 95,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            216,
                                        ),
                                    },
                                    condition: 97,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            223,
                                        ),
                                    },
                                    condition: 99,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            230,
                                        ),
                                    },
                                    condition: 105,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            241,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 28,
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
                                                243,
                                            ),
                                        ),
                                    ),
                                    initial_value: 108,
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
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
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
                                                11,
                                            ),
                                        ),
                                    ),
                                    initial_value: 4,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    condition: 9,
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
                                                syn_pattern_expr_idx: 4,
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
                                                26,
                                            ),
                                        ),
                                    ),
                                    initial_value: 13,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            33,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 5,
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
                                                35,
                                            ),
                                        ),
                                    ),
                                    initial_value: 17,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            42,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 6,
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
                                                44,
                                            ),
                                        ),
                                    ),
                                    initial_value: 21,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            51,
                                        ),
                                    },
                                    condition: 24,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            55,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 7,
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
                                                57,
                                            ),
                                        ),
                                    ),
                                    initial_value: 29,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            65,
                                        ),
                                    },
                                    condition: 32,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                69,
                                            ),
                                        },
                                        condition: Ok(
                                            35,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    73,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            1..15,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                204,
                                            ),
                                        },
                                        condition: Ok(
                                            93,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    208,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            15..20,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 109,
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
                                            ident: `cc_num`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `eff_holes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                10,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::UnitTypeVariant {
                                        path_expr_idx: 3,
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
                                            ident: `left_cc`,
                                            regional_token_idx: RegionalTokenIdx(
                                                25,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `right_cc`,
                                            regional_token_idx: RegionalTokenIdx(
                                                34,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `down_cc`,
                                            regional_token_idx: RegionalTokenIdx(
                                                43,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `lower_excess`,
                                            regional_token_idx: RegionalTokenIdx(
                                                56,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                79,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 9,
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
                                                78,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    8,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                80,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                86,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 10,
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
                                                85,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    10,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                87,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `a`,
                                            regional_token_idx: RegionalTokenIdx(
                                                89,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `end_tan`,
                                            regional_token_idx: RegionalTokenIdx(
                                                101,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `x`,
                                            regional_token_idx: RegionalTokenIdx(
                                                115,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `y`,
                                            regional_token_idx: RegionalTokenIdx(
                                                126,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `left_ymax`,
                                            regional_token_idx: RegionalTokenIdx(
                                                137,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `left_ymin`,
                                            regional_token_idx: RegionalTokenIdx(
                                                148,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `left_mid_y`,
                                            regional_token_idx: RegionalTokenIdx(
                                                159,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `right_ymax`,
                                            regional_token_idx: RegionalTokenIdx(
                                                169,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `right_ymin`,
                                            regional_token_idx: RegionalTokenIdx(
                                                180,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `right_mid_y`,
                                            regional_token_idx: RegionalTokenIdx(
                                                191,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                214,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 11,
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
                                                213,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    22,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                215,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                221,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 12,
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
                                                220,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    24,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                222,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                228,
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
                                                227,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    26,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                229,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `a`,
                                            regional_token_idx: RegionalTokenIdx(
                                                242,
                                            ),
                                        },
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
                                        6,
                                    ),
                                    SynPatternSymbol::Atom(
                                        7,
                                    ),
                                    SynPatternSymbol::Atom(
                                        8,
                                    ),
                                    SynPatternSymbol::Atom(
                                        10,
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
                                    SynPatternSymbol::Atom(
                                        19,
                                    ),
                                    SynPatternSymbol::Atom(
                                        20,
                                    ),
                                    SynPatternSymbol::Atom(
                                        21,
                                    ),
                                    SynPatternSymbol::Atom(
                                        22,
                                    ),
                                    SynPatternSymbol::Atom(
                                        24,
                                    ),
                                    SynPatternSymbol::Atom(
                                        26,
                                    ),
                                    SynPatternSymbol::Atom(
                                        28,
                                    ),
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `cc_num`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `eff_holes`,
                                        2,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `left_cc`,
                                        3,
                                    ),
                                ],
                                [
                                    (
                                        `right_cc`,
                                        4,
                                    ),
                                ],
                                [
                                    (
                                        `down_cc`,
                                        5,
                                    ),
                                ],
                                [
                                    (
                                        `lower_excess`,
                                        6,
                                    ),
                                ],
                                [
                                    (
                                        `_`,
                                        7,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `_`,
                                        8,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `a`,
                                        9,
                                    ),
                                ],
                                [
                                    (
                                        `end_tan`,
                                        10,
                                    ),
                                ],
                                [
                                    (
                                        `x`,
                                        11,
                                    ),
                                ],
                                [
                                    (
                                        `y`,
                                        12,
                                    ),
                                ],
                                [
                                    (
                                        `left_ymax`,
                                        13,
                                    ),
                                ],
                                [
                                    (
                                        `left_ymin`,
                                        14,
                                    ),
                                ],
                                [
                                    (
                                        `left_mid_y`,
                                        15,
                                    ),
                                ],
                                [
                                    (
                                        `right_ymax`,
                                        16,
                                    ),
                                ],
                                [
                                    (
                                        `right_ymin`,
                                        17,
                                    ),
                                ],
                                [
                                    (
                                        `right_mid_y`,
                                        18,
                                    ),
                                ],
                                [
                                    (
                                        `_`,
                                        19,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `_`,
                                        20,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `_`,
                                        21,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `a`,
                                        22,
                                    ),
                                ],
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
                                                    251,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `cc_num`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            11,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    251,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `eff_holes`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            26,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    251,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `left_cc`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            35,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    251,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `right_cc`,
                                            pattern_symbol_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            44,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    251,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `down_cc`,
                                            pattern_symbol_idx: 5,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            57,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    251,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `lower_excess`,
                                            pattern_symbol_idx: 6,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            90,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `a`,
                                            pattern_symbol_idx: 9,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            102,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `end_tan`,
                                            pattern_symbol_idx: 10,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            116,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `x`,
                                            pattern_symbol_idx: 11,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            127,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `y`,
                                            pattern_symbol_idx: 12,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            138,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `left_ymax`,
                                            pattern_symbol_idx: 13,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            149,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `left_ymin`,
                                            pattern_symbol_idx: 14,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            160,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `left_mid_y`,
                                            pattern_symbol_idx: 15,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            170,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `right_ymax`,
                                            pattern_symbol_idx: 16,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            181,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `right_ymin`,
                                            pattern_symbol_idx: 17,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            192,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    204,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `right_mid_y`,
                                            pattern_symbol_idx: 18,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            243,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    248,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `a`,
                                            pattern_symbol_idx: 22,
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
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 6,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 7,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 9,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
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
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 19,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 20,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 21,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 23,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 25,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Be,
                                syn_pattern_expr_idx: 27,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 28,
                            },
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 4,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 9,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 13,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 17,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 21,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 24,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 29,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 32,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 37,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 39,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 42,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 46,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 51,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 55,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 59,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 63,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 67,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 73,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 77,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 81,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 87,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 90,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 95,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 97,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 99,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 105,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 108,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 109,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 110,
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
                                9,
                                7,
                            ),
                            (
                                10,
                                8,
                            ),
                            (
                                11,
                                9,
                            ),
                            (
                                12,
                                10,
                            ),
                            (
                                13,
                                11,
                            ),
                            (
                                14,
                                12,
                            ),
                            (
                                15,
                                13,
                            ),
                            (
                                16,
                                14,
                            ),
                            (
                                17,
                                15,
                            ),
                            (
                                18,
                                16,
                            ),
                            (
                                22,
                                17,
                            ),
                        ],
                    },
                },
            },
        ),
    ),
]