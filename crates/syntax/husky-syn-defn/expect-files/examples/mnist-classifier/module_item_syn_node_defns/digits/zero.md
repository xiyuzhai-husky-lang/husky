[
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Fugitive(
                FugitiveSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Fugitive(
                                FugitiveSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Some(
            ItemSynNodeDefn {
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
                                                                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                                FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
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
                                            ident: `almost_closed`,
                                            regional_token_idx: RegionalTokenIdx(
                                                6,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
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
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Fugitive(
                FugitiveSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Fugitive(
                                FugitiveSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 13,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
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
                                        2,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            4,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 2,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ropd: 3,
                                },
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    opd: 4,
                                },
                                SynExprData::Binary {
                                    lopd: 5,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ropd: 6,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `cc`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 8,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                },
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    opd: 9,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        15,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 42,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 10,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ropd: 11,
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        1..3,
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
                                            1,
                                        ),
                                    },
                                    condition: 7,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 12,
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
                                data: [],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [],
                        },
                        syn_pattern_expr_roots: [],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 7,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 12,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 13,
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
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Fugitive(
                FugitiveSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Fugitive(
                                FugitiveSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 91,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                                                        ident: `Zero`,
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
                                                        ident: `Zero`,
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
                                                                        ident: `Zero`,
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
                                                            path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                        3,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `raw_contours`,
                                        regional_token_idx: RegionalTokenIdx(
                                            4,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 2,
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
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 3,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ropd: 4,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 6,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `n`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        21,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 43,
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
                                        20,
                                    ),
                                    ropd: 9,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 11,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        27,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 12,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 13,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                },
                                SynExprData::Be {
                                    src: 14,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        29,
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
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 16,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            37,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        41,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 17,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                    ropd: 18,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 21,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 22,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 23,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 24,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            53,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 25,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            57,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `c`,
                                    regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        63,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 27,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    ropd: 28,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 8,
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
                                    path_expr_idx: 9,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 10,
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
                                        75,
                                    ),
                                    items: [],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 31,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 32,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    74,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 33,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        77,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 11,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 36,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            82,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 38,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `rel_norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            86,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 40,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change_norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            90,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        94,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            5,
                                        ),
                                    ),
                                ),
                                SynExprData::FunctionCall {
                                    function: 35,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                    items: [
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 37,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        83,
                                                    ),
                                                ),
                                            },
                                        ),
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 39,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        87,
                                                    ),
                                                ),
                                            },
                                        ),
                                        RegularOrVariadic(
                                            SynRegularOrVariadicCallListItem {
                                                argument_expr_idx: 41,
                                                separator: Comma(
                                                    RegionalTokenIdx(
                                                        91,
                                                    ),
                                                ),
                                            },
                                        ),
                                        Keyed(
                                            SynKeyedCallListItem {
                                                key_regional_token_idx: RegionalTokenIdx(
                                                    92,
                                                ),
                                                key: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 442,
                                                        },
                                                    ),
                                                ),
                                                argument_expr_idx: 42,
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                },
                                SynExprData::Suffix {
                                    opd: 43,
                                    opr: UnveilOrComposeWithOption,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        98,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 45,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        99,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            100,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        102,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 46,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                    ropd: 47,
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
                                    owner: 49,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `eff_holes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            106,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 50,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            108,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        110,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 51,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 52,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        111,
                                    ),
                                },
                                SynExprData::Be {
                                    src: 53,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        112,
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
                                    path_expr_idx: 14,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 55,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `eff_holes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            117,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 56,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        118,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            119,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        121,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 57,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        120,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 58,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        122,
                                    ),
                                },
                                SynExprData::Be {
                                    src: 59,
                                    be_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    target: Ok(
                                        BePatternSynSyndicate {
                                            pattern_expr_root: BeSynPatternExprRoot {
                                                syn_pattern_expr_idx: 8,
                                            },
                                            variables: ArenaIdxRange(
                                                4..4,
                                            ),
                                        },
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 16,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 61,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        132,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `eff_holes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            133,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 62,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        134,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            135,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        137,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 63,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        136,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 64,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        138,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `major_hole`,
                                    regional_token_idx: RegionalTokenIdx(
                                        142,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 66,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 67,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        144,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            145,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `major_hole`,
                                    regional_token_idx: RegionalTokenIdx(
                                        151,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Suffix {
                                    opd: 69,
                                    opr: UnwrapOrComposeWithNot,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 70,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        153,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            154,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 68,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        146,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
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
                                    self_argument: 71,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        155,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            156,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        157,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        158,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 72,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        150,
                                    ),
                                    ropd: 73,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 17,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 75,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        163,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            164,
                                        ),
                                    },
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
                                    owner: 77,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        171,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            172,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 76,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        165,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymax`,
                                        regional_token_idx: RegionalTokenIdx(
                                            166,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        167,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        168,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 78,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        173,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            174,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        175,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        176,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 79,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        169,
                                    ),
                                    ropd: 80,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        180,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `b`,
                                    regional_token_idx: RegionalTokenIdx(
                                        182,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 82,
                                    opr: Closed(
                                        Div,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        181,
                                    ),
                                    ropd: 83,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ratio`,
                                    regional_token_idx: RegionalTokenIdx(
                                        184,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        186,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 85,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        185,
                                    ),
                                    ropd: 86,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `simp_zero_match`,
                                    regional_token_idx: RegionalTokenIdx(
                                        190,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 88,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        191,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            192,
                                        ),
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 20,
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
                                        8..21,
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
                                                2,
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
                                            ident: `open_one_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `open_one_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                23,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `Some`,
                                            regional_token_idx: RegionalTokenIdx(
                                                30,
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
                                            ident: `connected_components`,
                                            regional_token_idx: RegionalTokenIdx(
                                                35,
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
                                            ident: `open_one_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                45,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                65,
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
                                    parent: 7,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            66,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                67,
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
                                            ident: `fermi_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                71,
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
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `narrow_down`,
                                            regional_token_idx: RegionalTokenIdx(
                                                78,
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
                                                104,
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
                                                113,
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
                                            ident: `major_connected_component`,
                                            regional_token_idx: RegionalTokenIdx(
                                                115,
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
                                                124,
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
                                                131,
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
                                            ident: `major_line_segment_sketch`,
                                            regional_token_idx: RegionalTokenIdx(
                                                162,
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
                                            ident: `major_line_segment_sketch`,
                                            regional_token_idx: RegionalTokenIdx(
                                                170,
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
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                193,
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
                                    parent: 19,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            194,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                195,
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
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
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
                                                14,
                                            ),
                                        ),
                                    ),
                                    initial_value: 7,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                    },
                                    condition: 10,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    condition: 15,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            34,
                                        ),
                                    },
                                    condition: 19,
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
                                                44,
                                            ),
                                        ),
                                    ),
                                    initial_value: 26,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            60,
                                        ),
                                    },
                                    condition: 29,
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            64,
                                        ),
                                    },
                                    result: 30,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                        condition: Ok(
                                            5,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            1..8,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
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
                                                70,
                                            ),
                                        ),
                                    ),
                                    initial_value: 34,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 44,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            97,
                                        ),
                                    },
                                    condition: 48,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            103,
                                        ),
                                    },
                                    condition: 54,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            114,
                                        ),
                                    },
                                    condition: 60,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            128,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 9,
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
                                                130,
                                            ),
                                        ),
                                    ),
                                    initial_value: 65,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            139,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 10,
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
                                                141,
                                            ),
                                        ),
                                    ),
                                    initial_value: 74,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            159,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 11,
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
                                                161,
                                            ),
                                        ),
                                    ),
                                    initial_value: 81,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            177,
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
                                                179,
                                            ),
                                        ),
                                    ),
                                    initial_value: 84,
                                },
                                SynStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            183,
                                        ),
                                    },
                                    condition: 87,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            187,
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
                                                189,
                                            ),
                                        ),
                                    ),
                                    initial_value: 89,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 90,
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
                                            ident: `n`,
                                            regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                32,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 4,
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
                                                31,
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
                                                33,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `c`,
                                            regional_token_idx: RegionalTokenIdx(
                                                43,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `simp_zero_match`,
                                            regional_token_idx: RegionalTokenIdx(
                                                69,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::UnitTypeVariant {
                                        path_expr_idx: 13,
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
                                            ident: `_`,
                                            regional_token_idx: RegionalTokenIdx(
                                                126,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::TupleTypeVariant {
                                        path_expr_idx: 15,
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
                                                125,
                                            ),
                                        ),
                                        fields: PunctuatedSmallList {
                                            elements: [
                                                SynPatternComponent(
                                                    7,
                                                ),
                                            ],
                                            separators: [],
                                            phantom: PhantomData<husky_syn_expr::error::SynExprError>,
                                        },
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                127,
                                            ),
                                        ),
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `major_hole`,
                                            regional_token_idx: RegionalTokenIdx(
                                                129,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `a`,
                                            regional_token_idx: RegionalTokenIdx(
                                                140,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `b`,
                                            regional_token_idx: RegionalTokenIdx(
                                                160,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `ratio`,
                                            regional_token_idx: RegionalTokenIdx(
                                                178,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `a`,
                                            regional_token_idx: RegionalTokenIdx(
                                                188,
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
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `n`,
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
                                        `c`,
                                        3,
                                    ),
                                ],
                                [
                                    (
                                        `simp_zero_match`,
                                        4,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `_`,
                                        5,
                                    ),
                                ],
                                [],
                                [
                                    (
                                        `major_hole`,
                                        6,
                                    ),
                                ],
                                [
                                    (
                                        `a`,
                                        7,
                                    ),
                                ],
                                [
                                    (
                                        `b`,
                                        8,
                                    ),
                                ],
                                [
                                    (
                                        `ratio`,
                                        9,
                                    ),
                                ],
                                [
                                    (
                                        `a`,
                                        10,
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
                                            14,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    68,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `n`,
                                            pattern_symbol_idx: 1,
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
                                                    68,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `c`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            70,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    196,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `simp_zero_match`,
                                            pattern_symbol_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            130,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    196,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `major_hole`,
                                            pattern_symbol_idx: 6,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            141,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    196,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `a`,
                                            pattern_symbol_idx: 7,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            161,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    196,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `b`,
                                            pattern_symbol_idx: 8,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            179,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    196,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `ratio`,
                                            pattern_symbol_idx: 9,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            189,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    196,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `a`,
                                            pattern_symbol_idx: 10,
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
                                kind: SynPatternExprRootKind::Be,
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
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 12,
                            },
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 13,
                            },
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 7,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 10,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 15,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 19,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 26,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 29,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 30,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 34,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 44,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 48,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 54,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 60,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 65,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 74,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 81,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 84,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 87,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 89,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 90,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 91,
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
                                6,
                                4,
                            ),
                            (
                                7,
                                5,
                            ),
                            (
                                8,
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
                        ],
                    },
                },
            },
        ),
    ),
]