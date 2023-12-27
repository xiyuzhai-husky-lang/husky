[
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Type(
                TypeSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Type(
                                TypeSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Type(
                TypeSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Type(
                                TypeSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
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
                                        path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
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
                                                                        path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
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
                                                        ident: `RawContour`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        ident: `ct`,
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
                                                    `ct`,
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
                                                        ident: `ct`,
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
                                                            path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
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
                                    ident: `ct`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `ct`,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `contour_len`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `len`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 1,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 3,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ropd: 4,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `len`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 6,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ropd: 7,
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
                                            7,
                                        ),
                                    },
                                    condition: 5,
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
                                            ident: `len`,
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
                                        `len`,
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
                                            ident: `ct`,
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
                                                    14,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `len`,
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
                                syn_expr_idx: 5,
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
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Type(
                TypeSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Type(
                                TypeSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
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
                                        path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
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
                body: 52,
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
                                                                        path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
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
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        ident: `r32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::raw_bits::r32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `r32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::raw_bits::r32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `r32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        ident: `a`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternExprData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
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
                                                    `a`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `x`,
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
                                                        ident: `a`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `x`,
                                                        pattern_symbol_idx: 2,
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
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 2,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 3,
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
                                                            path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
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
                                    ident: `x`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `x`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 1,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ropd: 2,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `x`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `x`,
                                    },
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    item: 3,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `x`,
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    inherited_syn_symbol_idx: 2,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `x`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        19,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 6,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    ropd: 7,
                                },
                                SynExprData::Binary {
                                    lopd: 4,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ropd: 5,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    item: 8,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 9,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ropd: 10,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `a`,
                                    },
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    item: 11,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 12,
                                    opr: Closed(
                                        BitAnd,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    ropd: 13,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        34,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 15,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ropd: 16,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    item: 17,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
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
                                SynExprData::Binary {
                                    lopd: 20,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ropd: 21,
                                },
                                SynExprData::Binary {
                                    lopd: 18,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ropd: 19,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    item: 22,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 23,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ropd: 24,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `a`,
                                    },
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    item: 25,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 26,
                                    opr: Closed(
                                        BitAnd,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ropd: 27,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `z`,
                                    regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 29,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    ropd: 30,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `z`,
                                    regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 32,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                    ropd: 33,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        61,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 35,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        60,
                                    ),
                                    ropd: 36,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                    item: 37,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        67,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 40,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    ropd: 41,
                                },
                                SynExprData::Binary {
                                    lopd: 38,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                    ropd: 39,
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        64,
                                    ),
                                    item: 42,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 43,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                    ropd: 44,
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `a`,
                                    },
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    item: 45,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `z`,
                                    regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 46,
                                    opr: Closed(
                                        BitAnd,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    ropd: 47,
                                },
                                SynExprData::Binary {
                                    lopd: 48,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ropd: 49,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `y`,
                                    regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        3..7,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 34,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 50,
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
                                    initial_value: 14,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
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
                                                25,
                                            ),
                                        ),
                                    ),
                                    initial_value: 28,
                                },
                                SynStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            43,
                                        ),
                                    },
                                    condition: Ok(
                                        31,
                                    ),
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    47,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        1..3,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            70,
                                        ),
                                    },
                                    result: 51,
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
                                            ident: `y`,
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
                                                        23,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `z`,
                                            regional_token_idx: RegionalTokenIdx(
                                                24,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Move,
                                    Move,
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
                                        `y`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `z`,
                                        2,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
                                    Mut,
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
                                            ident: `a`,
                                        },
                                    },
                                    InheritedSynSymbol {
                                        parent_symbol_idx: Current(
                                            2,
                                        ),
                                        modifier: Pure,
                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                            ident: `x`,
                                        },
                                    },
                                ],
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
                                                    72,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `y`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            25,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    72,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `z`,
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
                                syn_expr_idx: 14,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 28,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 34,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 50,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 51,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 52,
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
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Fugitive(
                FugitiveSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Fugitive(
                                FugitiveSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
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
                body: 123,
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
                                                                        path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
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
                                                            TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BinaryImage28`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `ConnectedComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
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
                                            data: [
                                                SynPatternExprData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `img`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
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
                                                    `img`,
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
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `img`,
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
                                                    ty_expr_idx: 1,
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
                                            syn_expr_idx: 1,
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
                                                            path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
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
                                SynExprData::List {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    items: [],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `img`,
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `img`,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 5,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `clone`,
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident: `j`,
                                    frame_var_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        7,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        23,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            30,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 7,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ropd: 8,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `unsearched`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        7,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 10,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 11,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `unsearched`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        7,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 13,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 14,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 16,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ctz`,
                                        regional_token_idx: RegionalTokenIdx(
                                            43,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                },
                                SynExprData::AssociatedItem {
                                    parent_expr_idx: 2,
                                    parent_path: MajorItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                    ),
                                    colon_colon_regional_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            51,
                                        ),
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `new_zeros`,
                                        regional_token_idx: RegionalTokenIdx(
                                            52,
                                        ),
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 18,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        7,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        64,
                                    ),
                                    LiteralData::Integer(
                                        R32(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `shift`,
                                    regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 24,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                    ropd: 25,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 20,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 21,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 22,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 23,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    63,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 26,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 27,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    ropd: 28,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        72,
                                    ),
                                    LiteralData::Bool(
                                        False,
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `flag`,
                                    regional_token_idx: RegionalTokenIdx(
                                        75,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Prefix {
                                    opr: Not,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    opd: 31,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `flag`,
                                    regional_token_idx: RegionalTokenIdx(
                                        77,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        79,
                                    ),
                                    LiteralData::Bool(
                                        True,
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 33,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                    ropd: 34,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        7,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        88,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            30,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        90,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        86,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 37,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ropd: 38,
                                },
                                SynExprData::Binary {
                                    lopd: 39,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                    ropd: 40,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        97,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        99,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 43,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        98,
                                    ),
                                    ropd: 44,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 42,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 45,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `img`,
                                    regional_token_idx: RegionalTokenIdx(
                                        108,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `img`,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        112,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 49,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        111,
                                    ),
                                    ropd: 50,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 48,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 51,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        117,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 53,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 54,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        118,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `old_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 47,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        107,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 52,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    114,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 55,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        119,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 56,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    ropd: 57,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `new_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        122,
                                    ),
                                    current_syn_symbol_idx: 10,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                                SynExprData::Prefix {
                                    opr: Not,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        121,
                                    ),
                                    opd: 59,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `old_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        126,
                                    ),
                                    current_syn_symbol_idx: 9,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 8,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `new_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        128,
                                    ),
                                    current_syn_symbol_idx: 10,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 61,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        127,
                                    ),
                                    ropd: 62,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `flag`,
                                    regional_token_idx: RegionalTokenIdx(
                                        130,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        132,
                                    ),
                                    LiteralData::Bool(
                                        False,
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 64,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        131,
                                    ),
                                    ropd: 65,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        133,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        135,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        137,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 68,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        136,
                                    ),
                                    ropd: 69,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 67,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        134,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 70,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        138,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `new_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        140,
                                    ),
                                    current_syn_symbol_idx: 10,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 9,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 71,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        139,
                                    ),
                                    ropd: 72,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        142,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        144,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        7,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 74,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        143,
                                    ),
                                    ropd: 75,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        149,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        151,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 77,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        150,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 78,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        152,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `img`,
                                    regional_token_idx: RegionalTokenIdx(
                                        160,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `img`,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        162,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 81,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        161,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 82,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        163,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        165,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        167,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        169,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 85,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        168,
                                    ),
                                    ropd: 86,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 84,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        166,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 87,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        170,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `old_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        156,
                                    ),
                                    current_syn_symbol_idx: 11,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 10,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 80,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        159,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 83,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    164,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 88,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        171,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 89,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        157,
                                    ),
                                    ropd: 90,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `old_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        173,
                                    ),
                                    current_syn_symbol_idx: 11,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 10,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `new_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        175,
                                    ),
                                    current_syn_symbol_idx: 12,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 11,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 92,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        174,
                                    ),
                                    ropd: 93,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `flag`,
                                    regional_token_idx: RegionalTokenIdx(
                                        177,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        179,
                                    ),
                                    LiteralData::Bool(
                                        False,
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 95,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        178,
                                    ),
                                    ropd: 96,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        180,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        182,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 7,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 98,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        181,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 99,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        183,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `new_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        185,
                                    ),
                                    current_syn_symbol_idx: 12,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 11,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 100,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        184,
                                    ),
                                    ropd: 101,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        187,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        7,
                                    ),
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        189,
                                    ),
                                    ident: `k`,
                                    frame_var_symbol_idx: 13,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        104,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 103,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        188,
                                    ),
                                    ropd: 104,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        191,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            30,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 105,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        190,
                                    ),
                                    ropd: 106,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `unsearched`,
                                    regional_token_idx: RegionalTokenIdx(
                                        193,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `k`,
                                    regional_token_idx: RegionalTokenIdx(
                                        195,
                                    ),
                                    current_syn_symbol_idx: 13,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        104,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        200,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `k`,
                                    regional_token_idx: RegionalTokenIdx(
                                        202,
                                    ),
                                    current_syn_symbol_idx: 13,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        104,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 110,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        201,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 111,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        203,
                                    ),
                                },
                                SynExprData::Prefix {
                                    opr: Tilde,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        199,
                                    ),
                                    opd: 112,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 108,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        194,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 109,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        196,
                                    ),
                                },
                                SynExprData::Bracketed {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        198,
                                    ),
                                    item: 113,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        204,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 114,
                                    opr: AssignClosed(
                                        BitAnd,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        197,
                                    ),
                                    ropd: 115,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        205,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `mask`,
                                    regional_token_idx: RegionalTokenIdx(
                                        211,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 118,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        210,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 119,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        212,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 117,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        206,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `push`,
                                        regional_token_idx: RegionalTokenIdx(
                                            207,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        208,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 120,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        213,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        215,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        27..31,
                                    ),
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
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `BinaryImage28`,
                                            regional_token_idx: RegionalTokenIdx(
                                                50,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist::BinaryImage28`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `horizontal_extend`,
                                            regional_token_idx: RegionalTokenIdx(
                                                60,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `horizontal_extend`,
                                            regional_token_idx: RegionalTokenIdx(
                                                106,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `horizontal_extend`,
                                            regional_token_idx: RegionalTokenIdx(
                                                158,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `ConnectedComponent`,
                                            regional_token_idx: RegionalTokenIdx(
                                                209,
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
                            data: [
                                SynStmtData::Break {
                                    break_token: BreakRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            124,
                                        ),
                                    },
                                },
                                SynStmtData::Eval {
                                    expr_idx: 66,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 73,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            92,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 8,
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
                                                94,
                                            ),
                                        ),
                                    ),
                                    initial_value: 46,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            101,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 9,
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
                                                103,
                                            ),
                                        ),
                                    ),
                                    initial_value: 58,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                120,
                                            ),
                                        },
                                        condition: Ok(
                                            60,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    123,
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
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                125,
                                            ),
                                        },
                                        condition: Ok(
                                            63,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    129,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            2..4,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 97,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 102,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            146,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 10,
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
                                                148,
                                            ),
                                        ),
                                    ),
                                    initial_value: 79,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            153,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 11,
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
                                                155,
                                            ),
                                        ),
                                    ),
                                    initial_value: 91,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                172,
                                            ),
                                        },
                                        condition: Ok(
                                            94,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    176,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            8..10,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 35,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            80,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 7,
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
                                                83,
                                            ),
                                        ),
                                    ),
                                    initial_value: 36,
                                },
                                SynStmtData::ForExt {
                                    forext_token: ForextRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            85,
                                        ),
                                    },
                                    particulars: SynForextParticulars {
                                        forext_loop_var_regional_token_idx: RegionalTokenIdx(
                                            86,
                                        ),
                                        forext_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 249,
                                                },
                                            ),
                                        ),
                                        forext_loop_var_expr_idx: 39,
                                        bound_expr: 40,
                                        boundary_kind: UpperOpen,
                                    },
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    91,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        4..8,
                                    ),
                                },
                                SynStmtData::ForExt {
                                    forext_token: ForextRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            141,
                                        ),
                                    },
                                    particulars: SynForextParticulars {
                                        forext_loop_var_regional_token_idx: RegionalTokenIdx(
                                            142,
                                        ),
                                        forext_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 249,
                                                },
                                            ),
                                        ),
                                        forext_loop_var_expr_idx: 74,
                                        bound_expr: 75,
                                        boundary_kind: LowerClosed,
                                    },
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    145,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        10..13,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 116,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
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
                                                33,
                                            ),
                                        ),
                                    ),
                                    initial_value: 15,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            38,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 4,
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
                                                40,
                                            ),
                                        ),
                                    ),
                                    initial_value: 17,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            46,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 5,
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
                                                49,
                                            ),
                                        ),
                                    ),
                                    initial_value: 19,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 29,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
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
                                                syn_pattern_expr_idx: 6,
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
                                                71,
                                            ),
                                        ),
                                    ),
                                    initial_value: 30,
                                },
                                SynStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            73,
                                        ),
                                    },
                                    condition: Ok(
                                        32,
                                    ),
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    76,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        13..17,
                                    ),
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            186,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            189,
                                        ),
                                        for_between_loop_var_ident: `k`,
                                        for_between_loop_var_expr_idx: 104,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        103,
                                                    ),
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        106,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                step: Constant(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 13,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    192,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        17..18,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 121,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                    condition: Ok(
                                        12,
                                    ),
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    30,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        18..26,
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
                                                Some(
                                                    ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty: Some(
                                                3,
                                            ),
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                    initial_value: 4,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
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
                                                14,
                                            ),
                                        ),
                                    ),
                                    initial_value: 6,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            21,
                                        ),
                                        for_between_loop_var_ident: `j`,
                                        for_between_loop_var_expr_idx: 7,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: None,
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        8,
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
                                                    24,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        26..27,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            214,
                                        ),
                                    },
                                    result: 122,
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
                                            ident: `result`,
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
                                                        12,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `unsearched`,
                                            regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `a`,
                                            regional_token_idx: RegionalTokenIdx(
                                                32,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `shift`,
                                            regional_token_idx: RegionalTokenIdx(
                                                39,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        47,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `mask`,
                                            regional_token_idx: RegionalTokenIdx(
                                                48,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        69,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `flag`,
                                            regional_token_idx: RegionalTokenIdx(
                                                70,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        81,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `i`,
                                            regional_token_idx: RegionalTokenIdx(
                                                82,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `old_row`,
                                            regional_token_idx: RegionalTokenIdx(
                                                93,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `new_row`,
                                            regional_token_idx: RegionalTokenIdx(
                                                102,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `old_row`,
                                            regional_token_idx: RegionalTokenIdx(
                                                147,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `new_row`,
                                            regional_token_idx: RegionalTokenIdx(
                                                154,
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
                                    Pure,
                                    Move,
                                    Move,
                                    Move,
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
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `result`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `unsearched`,
                                        2,
                                    ),
                                ],
                                [
                                    (
                                        `a`,
                                        3,
                                    ),
                                ],
                                [
                                    (
                                        `shift`,
                                        4,
                                    ),
                                ],
                                [
                                    (
                                        `mask`,
                                        5,
                                    ),
                                ],
                                [
                                    (
                                        `flag`,
                                        6,
                                    ),
                                ],
                                [
                                    (
                                        `i`,
                                        7,
                                    ),
                                ],
                                [
                                    (
                                        `old_row`,
                                        8,
                                    ),
                                ],
                                [
                                    (
                                        `new_row`,
                                        9,
                                    ),
                                ],
                                [
                                    (
                                        `old_row`,
                                        10,
                                    ),
                                ],
                                [
                                    (
                                        `new_row`,
                                        11,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
                                    Mut,
                                    Pure,
                                    Pure,
                                    Mut,
                                    Mut,
                                    Mut,
                                    Pure,
                                    Pure,
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
                                            ident: `img`,
                                        },
                                    },
                                ],
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
                                                    216,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `result`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            14,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    216,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `unsearched`,
                                            pattern_symbol_idx: 2,
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
                                                    214,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `j`,
                                            expr_idx: 7,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            33,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    214,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `a`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            40,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    214,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `shift`,
                                            pattern_symbol_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            49,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    214,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `mask`,
                                            pattern_symbol_idx: 5,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            71,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    214,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `flag`,
                                            pattern_symbol_idx: 6,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            83,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `i`,
                                            pattern_symbol_idx: 7,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            94,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    141,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `old_row`,
                                            pattern_symbol_idx: 8,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            103,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    141,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `new_row`,
                                            pattern_symbol_idx: 9,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            148,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `old_row`,
                                            pattern_symbol_idx: 10,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            155,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    186,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `new_row`,
                                            pattern_symbol_idx: 11,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            193,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    205,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `k`,
                                            expr_idx: 104,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [
                                (
                                    LetPattern {
                                        pattern: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        ty: 3,
                                    },
                                    ArenaIdxRange(
                                        1..2,
                                    ),
                                ),
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        3..4,
                                    ),
                                ),
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        13..14,
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
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtType,
                                syn_expr_idx: 3,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 4,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 6,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 15,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 17,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 19,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 29,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 30,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 35,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 36,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 46,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 58,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 66,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 73,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 79,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 91,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 97,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 102,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 116,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 121,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 122,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 123,
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
                            (
                                4,
                                5,
                            ),
                            (
                                5,
                                6,
                            ),
                            (
                                6,
                                7,
                            ),
                            (
                                7,
                                8,
                            ),
                            (
                                8,
                                9,
                            ),
                            (
                                9,
                                10,
                            ),
                            (
                                10,
                                11,
                            ),
                            (
                                11,
                                12,
                            ),
                        ],
                    },
                },
            },
        ),
    ),
    (
        ItemSynNodePath::ImplBlock(
            ImplBlockSynNodePath::TraitForTypeImplBlock(
                TraitForTypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                TraitForTypeImplBlockSynNodePathData {
                                    path: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::connected_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TraitForTypeItem(
                TraitForTypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                TraitForTypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `visualize`,
                                                            item_kind: MethodFn,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
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
                body: 4,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlock {
                                                                                    data: TraitForTypeImplBlockPathData {
                                                                                        module_path: `mnist_classifier::connected_component`,
                                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                                        ty_sketch: TypeSketch::Path(
                                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                                        ),
                                                                                        disambiguator: 0,
                                                                                    },
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
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                    ident: `Visualize`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::visual::Visualize`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        4,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 2,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TraitForTypeItemPath(
                                                                            ItemPathId {
                                                                                data: ItemPathData::AssociatedItem(
                                                                                    AssociatedItemPathData::TraitForTypeItem(
                                                                                        TraitForTypeItemPathData {
                                                                                            impl_block: TraitForTypeImplBlock {
                                                                                                data: TraitForTypeImplBlockPathData {
                                                                                                    module_path: `mnist_classifier::connected_component`,
                                                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                                                    ty_sketch: TypeSketch::Path(
                                                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                                                    ),
                                                                                                    disambiguator: 0,
                                                                                                },
                                                                                            },
                                                                                            ident: `visualize`,
                                                                                            item_kind: MethodFn,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
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
                                                            TypePath(`core::visual::Html`, `Extern`),
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
                                                        ident: `Html`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::visual::Html`, `Extern`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TraitForTypeItem(
                                    TraitForTypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::AssociatedItem(
                                                                        AssociatedItemPathData::TraitForTypeItem(
                                                                            TraitForTypeItemPathData {
                                                                                impl_block: TraitForTypeImplBlock {
                                                                                    data: TraitForTypeImplBlockPathData {
                                                                                        module_path: `mnist_classifier::connected_component`,
                                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                                        ty_sketch: TypeSketch::Path(
                                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                                        ),
                                                                                        disambiguator: 0,
                                                                                    },
                                                                                },
                                                                                ident: `visualize`,
                                                                                item_kind: MethodFn,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
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
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        1,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 2,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `visualize`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        7,
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
                            data: [],
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
                            allow_self_type: True,
                            allow_self_value: True,
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
        ItemSynNodePath::ImplBlock(
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(
                                        ItemPathId(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
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
                body: 4,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
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
                                                    4,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    5,
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
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
                                                FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
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
                                            ident: `find_raw_contours`,
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
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
                            allow_self_type: True,
                            allow_self_value: True,
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
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                body: 26,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                        ident: `EffHoles`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
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
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `raw_contours`,
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 2,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `collect_leashes`,
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        11,
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
                                SynExprData::Prefix {
                                    opr: Tilde,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    opd: 4,
                                },
                                SynExprData::List {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    items: [],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                                SynExprData::Prefix {
                                    opr: Option,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    opd: 5,
                                },
                                SynExprData::ExplicitApplication {
                                    function_expr_idx: 6,
                                    argument_expr_idx: 7,
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
                                SynExprData::CurrentSynSymbol {
                                    ident: `raw_contours`,
                                    regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 10,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `pop_with_largest_opt_f32`,
                                        regional_token_idx: RegionalTokenIdx(
                                            26,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 11,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `matches`,
                                    regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `raw_contours`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 14,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `pop_with_largest_opt_f32`,
                                        regional_token_idx: RegionalTokenIdx(
                                            37,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 15,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 13,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `push`,
                                        regional_token_idx: RegionalTokenIdx(
                                            33,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 16,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `matches`,
                                    regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `raw_contours`,
                                    regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 19,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `pop_with_largest_opt_f32`,
                                        regional_token_idx: RegionalTokenIdx(
                                            48,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 20,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 18,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `push`,
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 21,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `matches`,
                                    regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 23,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 24,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        1..7,
                                    ),
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
                                                20,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `hole_tmpl`,
                                            regional_token_idx: RegionalTokenIdx(
                                                28,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `hole_tmpl`,
                                            regional_token_idx: RegionalTokenIdx(
                                                39,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `hole_tmpl`,
                                            regional_token_idx: RegionalTokenIdx(
                                                50,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `FunctionFn`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `EffHoles`,
                                            regional_token_idx: RegionalTokenIdx(
                                                54,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                    initial_value: 3,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
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
                                                Some(
                                                    ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty: Some(
                                                8,
                                            ),
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                21,
                                            ),
                                        ),
                                    ),
                                    initial_value: 9,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 12,
                                    eol_semicolon: Ok(
                                        Some(
                                            EolSemicolonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    30,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 17,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 22,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            53,
                                        ),
                                    },
                                    result: 25,
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
                                            ident: `raw_contours`,
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
                                                        13,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `matches`,
                                            regional_token_idx: RegionalTokenIdx(
                                                14,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Move,
                                    Move,
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
                                        `raw_contours`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `matches`,
                                        2,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
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
                                                    58,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `raw_contours`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            15,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    58,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `matches`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: True,
                            allow_self_value: True,
                            pattern_ty_constraints: [
                                (
                                    LetPattern {
                                        pattern: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        ty: 8,
                                    },
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
                            SynPatternExprRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 2,
                            },
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 3,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtType,
                                syn_expr_idx: 8,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 9,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 12,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 17,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 22,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 25,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 26,
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
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`, `MemoizedField`),
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
                body: 24,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`, `MemoizedField`),
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
                                                            4,
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`, `MemoizedField`),
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
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 2,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `raw_contours`,
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `raw_contours`,
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 4,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ropd: 5,
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 6,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 7,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ropd: 8,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `raw_contours`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        5,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 10,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 11,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 12,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 13,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            33,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_hole_ilen`,
                                    regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `hole_ilen`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 15,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    ropd: 16,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_hole_ilen`,
                                    regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `hole_ilen`,
                                    regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 18,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    ropd: 19,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_hole_ilen`,
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
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
                                SynExprData::Binary {
                                    lopd: 21,
                                    opr: As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ropd: 22,
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        4..8,
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
                                                47,
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
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 20,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
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
                                                25,
                                            ),
                                        ),
                                    ),
                                    initial_value: 14,
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                36,
                                            ),
                                        },
                                        condition: Ok(
                                            17,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    40,
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
                                                8,
                                            ),
                                        ),
                                    ),
                                    initial_value: 3,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: 5,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        4,
                                                    ),
                                                    kind: LowerOpen,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        8,
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
                                                    22,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        2..4,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    result: 23,
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
                                            ident: `max_hole_ilen`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `raw_contours`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `hole_ilen`,
                                            regional_token_idx: RegionalTokenIdx(
                                                24,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Move,
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
                                        3,
                                    ),
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `max_hole_ilen`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `raw_contours`,
                                        2,
                                    ),
                                ],
                                [
                                    (
                                        `hole_ilen`,
                                        3,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
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
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            4,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    48,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `max_hole_ilen`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            8,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    48,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `raw_contours`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            23,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    44,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i`,
                                            expr_idx: 5,
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
                                                    44,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `hole_ilen`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: True,
                            allow_self_value: True,
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
                                syn_expr_idx: 3,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 14,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 20,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 23,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 24,
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
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`, `MemoizedField`),
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
                body: 20,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`, `MemoizedField`),
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
                                                            4,
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`, `MemoizedField`),
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
                                                TypePath(`core::num::i32`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        4,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 3,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ropd: 4,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            29,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 5,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ropd: 6,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        21,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        4,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 10,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 11,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 12,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `span`,
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
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 8,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 13,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 14,
                                    opr: Assign,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ropd: 15,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `max_row`,
                                    regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
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
                                SynExprData::Binary {
                                    lopd: 17,
                                    opr: As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    ropd: 18,
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
                                            ident: `i32`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::i32`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `f32`,
                                            regional_token_idx: RegionalTokenIdx(
                                                35,
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
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 16,
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
                                                Some(
                                                    ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty: Some(
                                                1,
                                            ),
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                6,
                                            ),
                                        ),
                                    ),
                                    initial_value: 2,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: 4,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        3,
                                                    ),
                                                    kind: LowerOpen,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        6,
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
                                            32,
                                        ),
                                    },
                                    result: 19,
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
                                            ident: `max_row`,
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
                                        `max_row`,
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
                                                    36,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `max_row`,
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
                                                    32,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i`,
                                            expr_idx: 4,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: True,
                            allow_self_value: True,
                            pattern_ty_constraints: [
                                (
                                    LetPattern {
                                        pattern: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        ty: 1,
                                    },
                                    ArenaIdxRange(
                                        1..2,
                                    ),
                                ),
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
                                kind: SynExprRootKind::LetStmtType,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 16,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 19,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 20,
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
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`, `MemoizedField`),
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
                body: 17,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`, `MemoizedField`),
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
                                                            4,
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`, `MemoizedField`),
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
                                        7,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        3,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 2,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ropd: 3,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        11,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            29,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 4,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ropd: 5,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        15,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 7,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
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
                                    owner: 8,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 9,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 10,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `span`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 11,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ropd: 12,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
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
                                SynExprData::Binary {
                                    lopd: 14,
                                    opr: As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ropd: 15,
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
                                            ident: `f32`,
                                            regional_token_idx: RegionalTokenIdx(
                                                28,
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
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 13,
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
                                            9,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: 3,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        2,
                                                    ),
                                                    kind: LowerOpen,
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
                                    for_loop_var_symbol_idx: 2,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
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
                                            25,
                                        ),
                                    },
                                    result: 16,
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
                                            ident: `row_span_sum`,
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
                                        `row_span_sum`,
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
                                            ident: `row_span_sum`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            13,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    25,
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
                            allow_self_type: True,
                            allow_self_value: True,
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
                                syn_expr_idx: 13,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
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
            },
        ),
    ),
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::distribution`, `MemoizedField`),
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
                body: 62,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::distribution`, `MemoizedField`),
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
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                        ident: `ConnectedComponentDistribution`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::distribution`, `MemoizedField`),
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
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            29,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 2,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ropd: 3,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        12,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 5,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 6,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 7,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        26,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 9,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    ropd: 10,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        30,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            29,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 12,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    ropd: 13,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        34,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 15,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            36,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 16,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 17,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                                SynExprData::Prefix {
                                    opr: Not,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    opd: 18,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 20,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ropd: 21,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `height`,
                                    regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        53,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 23,
                                    opr: Closed(
                                        Div,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ropd: 24,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        58,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        60,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    ident: `i1`,
                                    frame_var_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        28,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        64,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `half_height`,
                                    regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 27,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    ropd: 28,
                                },
                                SynExprData::Binary {
                                    lopd: 29,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                    ropd: 30,
                                },
                                SynExprData::Binary {
                                    lopd: 31,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                    ropd: 32,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        70,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 34,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            72,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        28,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 35,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 36,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        75,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `upper_mass`,
                                    regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 37,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `co`,
                                        regional_token_idx: RegionalTokenIdx(
                                            77,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 38,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                    ropd: 39,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        84,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        86,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    ident: `i2`,
                                    frame_var_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        43,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        90,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `half_height`,
                                    regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 42,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                    ropd: 43,
                                },
                                SynExprData::Binary {
                                    lopd: 44,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    ropd: 45,
                                },
                                SynExprData::Binary {
                                    lopd: 46,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ropd: 47,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        96,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 49,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        97,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            98,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                    current_syn_symbol_idx: 8,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        43,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 50,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        99,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 51,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `lower_mass`,
                                    regional_token_idx: RegionalTokenIdx(
                                        94,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 52,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        102,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `co`,
                                        regional_token_idx: RegionalTokenIdx(
                                            103,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 53,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                    ropd: 54,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `row_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        111,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `upper_mass`,
                                    regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `lower_mass`,
                                    regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                    current_syn_symbol_idx: 7,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 6,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 56,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        108,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 57,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    110,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 58,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    112,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 59,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    114,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 60,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    116,
                                                ),
                                            ),
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        117,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        7..18,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `ConnectedComponentDistribution`,
                                            regional_token_idx: RegionalTokenIdx(
                                                107,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Break {
                                    break_token: BreakRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                11,
                                            ),
                                        },
                                        condition: Ok(
                                            8,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    18,
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
                                SynStmtData::Break {
                                    break_token: BreakRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            41,
                                        ),
                                    },
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                32,
                                            ),
                                        },
                                        condition: Ok(
                                            19,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    40,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            3..4,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 40,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 55,
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
                                SynStmtData::ForExt {
                                    forext_token: ForextRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    particulars: SynForextParticulars {
                                        forext_loop_var_regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                        forext_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 232,
                                                },
                                            ),
                                        ),
                                        forext_loop_var_expr_idx: 2,
                                        bound_expr: 3,
                                        boundary_kind: UpperOpen,
                                    },
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        2..3,
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
                                                23,
                                            ),
                                        ),
                                    ),
                                    initial_value: 11,
                                },
                                SynStmtData::ForExt {
                                    forext_token: ForextRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                    particulars: SynForextParticulars {
                                        forext_loop_var_regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                        forext_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 233,
                                                },
                                            ),
                                        ),
                                        forext_loop_var_expr_idx: 12,
                                        bound_expr: 13,
                                        boundary_kind: UpperOpen,
                                    },
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        4..5,
                                    ),
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
                                                44,
                                            ),
                                        ),
                                    ),
                                    initial_value: 22,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            48,
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
                                                50,
                                            ),
                                        ),
                                    ),
                                    initial_value: 25,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            54,
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
                                                57,
                                            ),
                                        ),
                                    ),
                                    initial_value: 26,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            59,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            62,
                                        ),
                                        for_between_loop_var_ident: `i1`,
                                        for_between_loop_var_expr_idx: 28,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        27,
                                                    ),
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        32,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                step: Constant(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 6,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    67,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        5..6,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            80,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSynSyndicate {
                                            syn_pattern_expr_root: LetSynPatternExprRoot {
                                                syn_pattern_expr_idx: 6,
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
                                                83,
                                            ),
                                        ),
                                    ),
                                    initial_value: 41,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            85,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            88,
                                        ),
                                        for_between_loop_var_ident: `i2`,
                                        for_between_loop_var_expr_idx: 43,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        42,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        47,
                                                    ),
                                                    kind: LowerClosed,
                                                },
                                                step: Constant(
                                                    -1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 8,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    93,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        6..7,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            106,
                                        ),
                                    },
                                    result: 61,
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
                                            ident: `row_start`,
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
                                                        21,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `row_end`,
                                            regional_token_idx: RegionalTokenIdx(
                                                22,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `height`,
                                            regional_token_idx: RegionalTokenIdx(
                                                43,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `half_height`,
                                            regional_token_idx: RegionalTokenIdx(
                                                49,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        55,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `upper_mass`,
                                            regional_token_idx: RegionalTokenIdx(
                                                56,
                                            ),
                                        },
                                    },
                                    SynPatternExprData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        81,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `lower_mass`,
                                            regional_token_idx: RegionalTokenIdx(
                                                82,
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
                                    Pure,
                                    Move,
                                    Move,
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
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `row_start`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `row_end`,
                                        2,
                                    ),
                                ],
                                [
                                    (
                                        `height`,
                                        3,
                                    ),
                                ],
                                [
                                    (
                                        `half_height`,
                                        4,
                                    ),
                                ],
                                [
                                    (
                                        `upper_mass`,
                                        5,
                                    ),
                                ],
                                [
                                    (
                                        `lower_mass`,
                                        6,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
                                    Mut,
                                    Pure,
                                    Pure,
                                    Mut,
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
                                                    118,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `row_start`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            23,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    118,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `row_end`,
                                            pattern_symbol_idx: 2,
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
                                                    118,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `height`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            50,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    118,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `half_height`,
                                            pattern_symbol_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            57,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    118,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `upper_mass`,
                                            pattern_symbol_idx: 5,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            68,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    80,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i1`,
                                            expr_idx: 28,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            83,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    118,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `lower_mass`,
                                            pattern_symbol_idx: 6,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            94,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    106,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i2`,
                                            expr_idx: 43,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: True,
                            allow_self_value: True,
                            pattern_ty_constraints: [
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        6..7,
                                    ),
                                ),
                                (
                                    LoopVariable,
                                    ArenaIdxRange(
                                        8..9,
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
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 11,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 22,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 25,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 26,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 40,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 41,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 55,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 61,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 62,
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
                                7,
                            ),
                        ],
                    },
                },
            },
        ),
    ),
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`, `MemoizedField`),
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
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`, `MemoizedField`),
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
                                                            4,
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`, `MemoizedField`),
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
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        1,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `distribution`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 2,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `upper_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                },
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
                                SynExprData::Binary {
                                    lopd: 3,
                                    opr: As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    ropd: 4,
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
                                            ident: `f32`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
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
                            allow_self_type: True,
                            allow_self_value: True,
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
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
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
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
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
                                                            4,
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
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
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        1,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `distribution`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 2,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `lower_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                },
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
                                SynExprData::Binary {
                                    lopd: 3,
                                    opr: As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    ropd: 4,
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
                                            ident: `f32`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
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
                            allow_self_type: True,
                            allow_self_value: True,
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
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`, `MethodFn`),
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
                body: 30,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`, `MethodFn`),
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
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
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
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
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
                                                        ident: `k`,
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
                                                    `k`,
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
                                                        ident: `k`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 1,
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
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 2,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`, `MethodFn`),
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
                                SynExprData::InheritedSynSymbol {
                                    ident: `k`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `k`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 2,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ropd: 3,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        18,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            29,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 6,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ropd: 7,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        21,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 10,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 11,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ident: `j`,
                                    frame_var_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        14,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `k`,
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `k`,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 13,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ropd: 14,
                                },
                                SynExprData::Binary {
                                    lopd: 15,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ropd: 16,
                                },
                                SynExprData::Binary {
                                    lopd: 17,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ropd: 18,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        40,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 20,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            42,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        14,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 21,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 22,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `top_k_row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 23,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `span`,
                                        regional_token_idx: RegionalTokenIdx(
                                            47,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 24,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ropd: 25,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `top_k_row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
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
                                SynExprData::Binary {
                                    lopd: 27,
                                    opr: As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ropd: 28,
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        4..10,
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
                                                53,
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
                            data: [
                                SynStmtData::Break {
                                    break_token: BreakRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                20,
                                            ),
                                        },
                                        condition: Ok(
                                            12,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    27,
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
                                    expr_idx: 26,
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
                                SynStmtData::Assert {
                                    assert_token: AssertRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    condition: 4,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
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
                                                13,
                                            ),
                                        ),
                                    ),
                                    initial_value: 5,
                                },
                                SynStmtData::ForExt {
                                    forext_token: ForextRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    particulars: SynForextParticulars {
                                        forext_loop_var_regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                        forext_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 249,
                                                },
                                            ),
                                        ),
                                        forext_loop_var_expr_idx: 6,
                                        bound_expr: 7,
                                        boundary_kind: UpperOpen,
                                    },
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
                                        2..3,
                                    ),
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            29,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            32,
                                        ),
                                        for_between_loop_var_ident: `j`,
                                        for_between_loop_var_expr_idx: 14,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        13,
                                                    ),
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        18,
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
                                                    37,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        3..4,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            50,
                                        ),
                                    },
                                    result: 29,
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
                                            ident: `top_k_row_span_sum`,
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
                                                        11,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `i`,
                                            regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Move,
                                    Move,
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
                                        `top_k_row_span_sum`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `i`,
                                        2,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
                                    Mut,
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
                                            ident: `k`,
                                        },
                                    },
                                ],
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
                                                    54,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `top_k_row_span_sum`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            13,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    54,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `i`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            38,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    50,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `j`,
                                            expr_idx: 14,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: True,
                            allow_self_value: True,
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
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 4,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 5,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 26,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 29,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 30,
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
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`, `MethodFn`),
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
                body: 30,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 377,
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `ConnectedComponent`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`, `MethodFn`),
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
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
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
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
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
                                                        ident: `k`,
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
                                                    `k`,
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
                                                        ident: `k`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 1,
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
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 2,
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
                            ItemSynNodePath::AssociatedItem(
                                AssociatedItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssociatedItem(
                                                AssociatedItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`, `MethodFn`),
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
                                SynExprData::InheritedSynSymbol {
                                    ident: `k`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `k`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 2,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ropd: 3,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        18,
                                    ),
                                    LiteralData::Integer(
                                        UnspecifiedRegular(
                                            29,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 6,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ropd: 7,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        21,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 10,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 11,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ident: `j`,
                                    frame_var_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        14,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `k`,
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                        ident: `k`,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 13,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ropd: 14,
                                },
                                SynExprData::Binary {
                                    lopd: 15,
                                    opr: Closed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ropd: 16,
                                },
                                SynExprData::Binary {
                                    lopd: 17,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    ropd: 18,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        40,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 20,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `mask`,
                                        regional_token_idx: RegionalTokenIdx(
                                            42,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `j`,
                                    regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                        14,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 21,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 22,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `top_k_row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 23,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `right_mass`,
                                        regional_token_idx: RegionalTokenIdx(
                                            47,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 24,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ropd: 25,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `top_k_row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
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
                                SynExprData::Binary {
                                    lopd: 27,
                                    opr: As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ropd: 28,
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        4..10,
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
                                                53,
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
                            data: [
                                SynStmtData::Break {
                                    break_token: BreakRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                20,
                                            ),
                                        },
                                        condition: Ok(
                                            12,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    27,
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
                                    expr_idx: 26,
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
                                SynStmtData::Assert {
                                    assert_token: AssertRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    condition: 4,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
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
                                                13,
                                            ),
                                        ),
                                    ),
                                    initial_value: 5,
                                },
                                SynStmtData::ForExt {
                                    forext_token: ForextRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    particulars: SynForextParticulars {
                                        forext_loop_var_regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                        forext_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 249,
                                                },
                                            ),
                                        ),
                                        forext_loop_var_expr_idx: 6,
                                        bound_expr: 7,
                                        boundary_kind: UpperOpen,
                                    },
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
                                        2..3,
                                    ),
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            29,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            32,
                                        ),
                                        for_between_loop_var_ident: `j`,
                                        for_between_loop_var_expr_idx: 14,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        13,
                                                    ),
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        18,
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
                                                    37,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        3..4,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            50,
                                        ),
                                    },
                                    result: 29,
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
                                            ident: `top_k_row_span_sum`,
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
                                                        11,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `i`,
                                            regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Move,
                                    Move,
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
                                        `top_k_row_span_sum`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `i`,
                                        2,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
                                    Mut,
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
                                            ident: `k`,
                                        },
                                    },
                                ],
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
                                                    54,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `top_k_row_span_sum`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            13,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    54,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `i`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbol {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            38,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    50,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `j`,
                                            expr_idx: 14,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: True,
                            allow_self_value: True,
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
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::Condition,
                                syn_expr_idx: 4,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 5,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 26,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 29,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 30,
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