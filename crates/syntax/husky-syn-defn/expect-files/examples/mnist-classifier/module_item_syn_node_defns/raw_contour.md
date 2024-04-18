```rust
[
    (
        ItemSynNodePath::ImplBlock(
            ImplBlockSynNodePath::TraitForTypeImplBlock(
                TraitForTypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                TraitForTypeImplBlockSynNodePathData {
                                    path: TraitForTypeImplBlockPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
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
        ItemSynNodePath::AssocItem(
            AssocItemSynNodePath::TraitForTypeItem(
                TraitForTypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssocItem(
                            AssocItemSynNodePathData::TraitForTypeItem(
                                TraitForTypeItemSynNodePathData {
                                    disambiguated_item_path: DisambiguatedItemPath {
                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                            `<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`,
                                            TraitItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
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
                body: 3,
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
                                                                                path: TraitForTypeImplBlockPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
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
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
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
                                                                    ident: `RawContour`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        4,
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
                                                symbol_region: VariableRegionData {
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
                                                        syn_expr_idx: 0,
                                                    },
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
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`,
                                                                            TraitItemKind::MethodRitchie(
                                                                                RitchieItemKind::Fn,
                                                                            ),
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
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::visual::Visual`, `Extern`),
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
                                                        ident: `Visual`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::visual::Visual`, `Extern`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::Defn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TraitForTypeItem(
                                    TraitForTypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                `<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`,
                                                                TraitItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
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
                                        6,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 0,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                },
                                SynExprData::EmptyHtmlTag {
                                    empty_html_bra_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                    function_ident: IdentRegionalToken {
                                        ident: `Contour`,
                                        regional_token_idx: RegionalTokenIdx(
                                            2,
                                        ),
                                    },
                                    arguments: [
                                        SynHtmlArgumentExpr::Expanded {
                                            property_ident: IdentRegionalToken {
                                                ident: `points`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    3,
                                                ),
                                            },
                                            eq: EqRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            lcurl: InlineLcurlRegionalToken(
                                                RegionalTokenIdx(
                                                    5,
                                                ),
                                            ),
                                            expr: 1,
                                            rcurl: InlineRcurlRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                    ],
                                    empty_html_ket: EmptyHtmlKetRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        0..1,
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
                        symbol_region: VariableRegionData {
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
                                kind: SynExprRootKind::HtmlArgumentExpr,
                                syn_expr_idx: 1,
                            },
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
                                                value: 285,
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
        ItemSynNodePath::AssocItem(
            AssocItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssocItem(
                            AssocItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    disambiguated_item_path: DisambiguatedItemPath {
                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::line_segment_sketch`, `MemoizedField`),
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
                                                                                            value: 285,
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
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                    ident: `RawContour`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: VariableRegionData {
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
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::line_segment_sketch`, `MemoizedField`),
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
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                        ident: `LineSegmentSketch`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
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
                                    symbol_region: VariableRegionData {
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
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::Defn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::line_segment_sketch`, `MemoizedField`),
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
                                SynExprData::PrincipalEntityPathAssocItem {
                                    parent_expr_idx: 0,
                                    parent_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                    colon_colon_regional_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            2,
                                        ),
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `new`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 3,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::FunctionApplicationOrCall {
                                    function: 0,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 1,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 2,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        0..1,
                                    ),
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
                                                1,
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
                        symbol_region: VariableRegionData {
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
        ItemSynNodePath::AssocItem(
            AssocItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssocItem(
                            AssocItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    disambiguated_item_path: DisambiguatedItemPath {
                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
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
                body: 55,
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
                                                                                            value: 285,
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
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                    ident: `RawContour`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: VariableRegionData {
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
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
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
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                        ident: `BoundingBox`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::Defn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::bounding_box`, `MemoizedField`),
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
                                        4,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 0,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 1,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 2,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `start_point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 4,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `start_point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 6,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `start_point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 8,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            30,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `start_point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 10,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            37,
                                        ),
                                    },
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        41,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 12,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            43,
                                        ),
                                    },
                                },
                                SynExprData::FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ident: `i`,
                                    frame_var_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentVariableKind::LoopVariable(
                                        14,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 13,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            45,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 14,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                    ropd: 15,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        52,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 17,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            54,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentVariableKind::LoopVariable(
                                        14,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 18,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 19,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `xmin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        60,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        64,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 22,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            66,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `xmin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 21,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `min`,
                                        regional_token_idx: RegionalTokenIdx(
                                            62,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 23,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 24,
                                    opr: SynBinaryOpr::AssignOrDefEq,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    ropd: 25,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `xmax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        70,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 28,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        75,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            76,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `xmax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 27,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        71,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            72,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 29,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        77,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 30,
                                    opr: SynBinaryOpr::AssignOrDefEq,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                    ropd: 31,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ymin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 34,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            86,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ymin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 33,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `min`,
                                        regional_token_idx: RegionalTokenIdx(
                                            82,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 35,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 36,
                                    opr: SynBinaryOpr::AssignOrDefEq,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        79,
                                    ),
                                    ropd: 37,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ymax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        90,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `point`,
                                    regional_token_idx: RegionalTokenIdx(
                                        94,
                                    ),
                                    current_syn_symbol_idx: 6,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 5,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 40,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            96,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ymax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 39,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `max`,
                                        regional_token_idx: RegionalTokenIdx(
                                            92,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        93,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 41,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        97,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 42,
                                    opr: SynBinaryOpr::AssignOrDefEq,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ropd: 43,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `xmin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        103,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `xmax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 46,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        102,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 47,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    104,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 48,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        106,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ymin`,
                                    regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ymax`,
                                    regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 50,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 51,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    111,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 52,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 45,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 49,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    107,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 53,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    114,
                                                ),
                                            ),
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        5..12,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `BoundingBox`,
                                            regional_token_idx: RegionalTokenIdx(
                                                99,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `ClosedRange`,
                                            regional_token_idx: RegionalTokenIdx(
                                                101,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `ClosedRange`,
                                            regional_token_idx: RegionalTokenIdx(
                                                108,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                            49,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                51,
                                            ),
                                        ),
                                    ),
                                    initial_value: 20,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 26,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 32,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 38,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 44,
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
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_expr_idx: 0,
                                            },
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
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                3,
                                            ),
                                        ),
                                    ),
                                    initial_value: 3,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                13,
                                            ),
                                        ),
                                    ),
                                    initial_value: 5,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                20,
                                            ),
                                        ),
                                    ),
                                    initial_value: 7,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            24,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                27,
                                            ),
                                        ),
                                    ),
                                    initial_value: 9,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                34,
                                            ),
                                        ),
                                    ),
                                    initial_value: 11,
                                },
                                SynStmtData::ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            38,
                                        ),
                                    },
                                    particulars: SynForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            39,
                                        ),
                                        for_between_loop_var_ident: `i`,
                                        for_between_loop_var_expr_idx: 14,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: None,
                                                    kind: LowerClosed,
                                                },
                                                final_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        15,
                                                    ),
                                                    kind: UpperOpen,
                                                },
                                                step: Constant(
                                                    1,
                                                ),
                                            },
                                        ),
                                    },
                                    for_loop_var_symbol_idx: 5,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    48,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        0..5,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            98,
                                        ),
                                    },
                                    result: 54,
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternExprRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `start_point`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
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
                                            ident: `xmin`,
                                            regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        18,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `xmax`,
                                            regional_token_idx: RegionalTokenIdx(
                                                19,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `ymin`,
                                            regional_token_idx: RegionalTokenIdx(
                                                26,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `ymax`,
                                            regional_token_idx: RegionalTokenIdx(
                                                33,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `point`,
                                            regional_token_idx: RegionalTokenIdx(
                                                50,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Pure,
                                    Move,
                                    Move,
                                    Move,
                                    Move,
                                    Pure,
                                ],
                            },
                            pattern_symbol_arena: Arena {
                                data: [
                                    SynPatternSymbol::Atom(
                                        0,
                                    ),
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
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `start_point`,
                                        0,
                                    ),
                                ],
                                [
                                    (
                                        `xmin`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `xmax`,
                                        2,
                                    ),
                                ],
                                [
                                    (
                                        `ymin`,
                                        3,
                                    ),
                                ],
                                [
                                    (
                                        `ymax`,
                                        4,
                                    ),
                                ],
                                [
                                    (
                                        `point`,
                                        5,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Pure,
                                    Mut,
                                    Mut,
                                    Mut,
                                    Mut,
                                    Pure,
                                ],
                            },
                        },
                        symbol_region: VariableRegionData {
                            inherited_syn_symbol_arena: Arena {
                                data: [],
                            },
                            current_syn_symbol_arena: Arena {
                                data: [
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            3,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    116,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `start_point`,
                                            pattern_symbol_idx: 0,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            13,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    116,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `xmin`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            20,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    116,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `xmax`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            27,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    116,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `ymin`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            34,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    116,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `ymax`,
                                            pattern_symbol_idx: 4,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            49,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    98,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i`,
                                            expr_idx: 14,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            51,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    98,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `point`,
                                            pattern_symbol_idx: 5,
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
                                        5..6,
                                    ),
                                ),
                            ],
                        },
                        syn_pattern_expr_roots: [
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 0,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 1,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 2,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 3,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 4,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 5,
                            },
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 3,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 5,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 7,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 9,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 11,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 20,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 26,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 32,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 38,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 44,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 54,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 55,
                            },
                        ],
                        has_self_lifetime: false,
                        has_self_place: false,
                        syn_pattern_to_current_syn_symbol_map: [
                            (
                                0,
                                0,
                            ),
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
                                6,
                            ),
                        ],
                    },
                },
            },
        ),
    ),
    (
        ItemSynNodePath::AssocItem(
            AssocItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssocItem(
                            AssocItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    disambiguated_item_path: DisambiguatedItemPath {
                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::relative_bounding_box`, `MemoizedField`),
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
                                                                                            value: 285,
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
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                    ident: `RawContour`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: VariableRegionData {
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
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::relative_bounding_box`, `MemoizedField`),
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
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                        ident: `RelativeBoundingBox`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::Defn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::relative_bounding_box`, `MemoizedField`),
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
                                    owner: 0,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `cc`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `raw_contours`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 2,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 3,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                                SynExprData::Field {
                                    owner: 4,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                    },
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 6,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 5,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `relative_bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 7,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        0..1,
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
                                    expr_idx: 8,
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
                            allow_self_type: True,
                            allow_self_value: True,
                            pattern_ty_constraints: [],
                        },
                        syn_pattern_expr_roots: [],
                        syn_expr_roots: [
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
                        syn_pattern_to_current_syn_symbol_map: [],
                    },
                },
            },
        ),
    ),
    (
        ItemSynNodePath::AssocItem(
            AssocItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssocItem(
                            AssocItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    disambiguated_item_path: DisambiguatedItemPath {
                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::contour_len`, `MemoizedField`),
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
                body: 65,
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
                                                                                            value: 285,
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
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                    ident: `RawContour`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: VariableRegionData {
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
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::contour_len`, `MemoizedField`),
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
                                                path_expr_idx: 0,
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
                                    symbol_region: VariableRegionData {
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
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::Defn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::contour_len`, `MemoizedField`),
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
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                    LiteralTokenData::Integer(
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
                                    frame_var_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LoopVariable(
                                        2,
                                    ),
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        11,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 3,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 1,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                    ropd: 2,
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 4,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 5,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ropd: 6,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        22,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 8,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            24,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LoopVariable(
                                        2,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        28,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 10,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ropd: 11,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 9,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 12,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        33,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 14,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            35,
                                        ),
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `i`,
                                    regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LoopVariable(
                                        2,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 15,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 16,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `b`,
                                    regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 18,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 19,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        47,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            48,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 20,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    ropd: 21,
                                },
                                SynExprData::Delimitered {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    item: 22,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `b`,
                                    regional_token_idx: RegionalTokenIdx(
                                        60,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 24,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            58,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 25,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            62,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 26,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    ropd: 27,
                                },
                                SynExprData::Delimitered {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    item: 28,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        63,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 23,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `abs`,
                                        regional_token_idx: RegionalTokenIdx(
                                            51,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 29,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        64,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `abs`,
                                        regional_token_idx: RegionalTokenIdx(
                                            65,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `contour_len`,
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 30,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    ropd: 31,
                                },
                                SynExprData::Binary {
                                    lopd: 32,
                                    opr: SynBinaryOpr::AssignClosed(
                                        BinaryClosedOpr::Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        40,
                                    ),
                                    ropd: 33,
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        71,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 35,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            73,
                                        ),
                                    },
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        75,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 37,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            77,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 38,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            79,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        83,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 39,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        82,
                                    ),
                                    ropd: 40,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 36,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 41,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        88,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 43,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            90,
                                        ),
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        92,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 44,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 45,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        93,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        97,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `b`,
                                    regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 47,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        98,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            99,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 48,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        102,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `x`,
                                        regional_token_idx: RegionalTokenIdx(
                                            103,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 49,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                    ropd: 50,
                                },
                                SynExprData::Delimitered {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    item: 51,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `a`,
                                    regional_token_idx: RegionalTokenIdx(
                                        111,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `b`,
                                    regional_token_idx: RegionalTokenIdx(
                                        115,
                                    ),
                                    current_syn_symbol_idx: 5,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 4,
                                    },
                                },
                                SynExprData::Field {
                                    owner: 53,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            113,
                                        ),
                                    },
                                },
                                SynExprData::Field {
                                    owner: 54,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            117,
                                        ),
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 55,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        114,
                                    ),
                                    ropd: 56,
                                },
                                SynExprData::Delimitered {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                    item: 57,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        118,
                                    ),
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 52,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `abs`,
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
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 58,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        119,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `abs`,
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
                                SynExprData::CurrentSynSymbol {
                                    ident: `contour_len`,
                                    regional_token_idx: RegionalTokenIdx(
                                        94,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 59,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    ropd: 60,
                                },
                                SynExprData::Binary {
                                    lopd: 61,
                                    opr: SynBinaryOpr::AssignClosed(
                                        BinaryClosedOpr::Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                    ropd: 62,
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `contour_len`,
                                    regional_token_idx: RegionalTokenIdx(
                                        124,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        3..9,
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
                                            19,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_expr_idx: 1,
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
                                                21,
                                            ),
                                        ),
                                    ),
                                    initial_value: 13,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            30,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_expr_idx: 2,
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
                                                32,
                                            ),
                                        ),
                                    ),
                                    initial_value: 17,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 34,
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
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_expr_idx: 0,
                                            },
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
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                4,
                                            ),
                                        ),
                                    ),
                                    initial_value: 0,
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
                                        for_between_loop_var_expr_idx: 2,
                                        range: Ok(
                                            SynForBetweenRange {
                                                initial_boundary: SynForBetweenLoopBoundary {
                                                    bound_expr: Some(
                                                        1,
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
                                    for_loop_var_symbol_idx: 1,
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        0..3,
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            68,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                70,
                                            ),
                                        ),
                                    ),
                                    initial_value: 42,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            85,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                87,
                                            ),
                                        ),
                                    ),
                                    initial_value: 46,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 63,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            123,
                                        ),
                                    },
                                    result: 64,
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternExprRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
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
                                            ident: `contour_len`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `a`,
                                            regional_token_idx: RegionalTokenIdx(
                                                20,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `b`,
                                            regional_token_idx: RegionalTokenIdx(
                                                31,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `a`,
                                            regional_token_idx: RegionalTokenIdx(
                                                69,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `b`,
                                            regional_token_idx: RegionalTokenIdx(
                                                86,
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
                                    Pure,
                                    Pure,
                                ],
                            },
                            pattern_symbol_arena: Arena {
                                data: [
                                    SynPatternSymbol::Atom(
                                        0,
                                    ),
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
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `contour_len`,
                                        0,
                                    ),
                                ],
                                [
                                    (
                                        `a`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `b`,
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
                                        `b`,
                                        4,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
                                    Pure,
                                    Pure,
                                    Pure,
                                    Pure,
                                ],
                            },
                        },
                        symbol_region: VariableRegionData {
                            inherited_syn_symbol_arena: Arena {
                                data: [],
                            },
                            current_syn_symbol_arena: Arena {
                                data: [
                                    CurrentSynSymbolEntry {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            4,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    125,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `contour_len`,
                                            pattern_symbol_idx: 0,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            19,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    68,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LoopVariable {
                                            ident: `i`,
                                            expr_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            21,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    68,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `a`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            32,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    68,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `b`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            70,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    125,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `a`,
                                            pattern_symbol_idx: 3,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            87,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    125,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `b`,
                                            pattern_symbol_idx: 4,
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
                                        1..2,
                                    ),
                                ),
                            ],
                        },
                        syn_pattern_expr_roots: [
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 0,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 1,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 2,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 3,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 4,
                            },
                        ],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 0,
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
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 34,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 42,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 46,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 63,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::ReturnExpr,
                                syn_expr_idx: 64,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 65,
                            },
                        ],
                        has_self_lifetime: false,
                        has_self_place: false,
                        syn_pattern_to_current_syn_symbol_map: [
                            (
                                0,
                                0,
                            ),
                            (
                                1,
                                2,
                            ),
                            (
                                2,
                                3,
                            ),
                            (
                                3,
                                4,
                            ),
                            (
                                4,
                                5,
                            ),
                        ],
                    },
                },
            },
        ),
    ),
    (
        ItemSynNodePath::AssocItem(
            AssocItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssocItem(
                            AssocItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    disambiguated_item_path: DisambiguatedItemPath {
                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                body: 18,
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
                                                                                            value: 285,
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
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                    ident: `RawContour`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
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
                                                symbol_region: VariableRegionData {
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
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::displacement`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
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
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
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
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            7,
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
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
                                                        ident: `Vector2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
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
                                                    0,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `start`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `end`,
                                                    1,
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
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbolEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `start`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSynSymbolEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `end`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
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
                                            0,
                                            0,
                                        ),
                                        (
                                            1,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::Defn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TypeItem(
                                    TypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TypeItem(
                                                    TypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::raw_contour::RawContour(0)>::displacement`, `MethodRitchie(
                                                                Fn,
                                                            )`),
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
                                        4,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 0,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 1,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 3,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::ParenateParameter {
                                        ident: `start`,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `N`,
                                    regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 5,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::RemEuclid,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ropd: 6,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 4,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        17,
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
                                SynExprData::SelfValue(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                ),
                                SynExprData::Field {
                                    owner: 9,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `points`,
                                        regional_token_idx: RegionalTokenIdx(
                                            27,
                                        ),
                                    },
                                },
                                SynExprData::InheritedSynSymbol {
                                    ident: `end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    inherited_syn_symbol_idx: 1,
                                    inherited_syn_symbol_kind: InheritedVariableKind::ParenateParameter {
                                        ident: `end`,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `N`,
                                    regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    current_syn_symbol_idx: 0,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 0,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 11,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::RemEuclid,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ropd: 12,
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 10,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 13,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ct_start`,
                                    regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `ct_end`,
                                    regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: CurrentVariableKind::LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 15,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `to`,
                                        regional_token_idx: RegionalTokenIdx(
                                            35,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 16,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                },
                                SynExprData::Block {
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
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_expr_idx: 0,
                                            },
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
                                            11,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                13,
                                            ),
                                        ),
                                    ),
                                    initial_value: 8,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
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
                                                24,
                                            ),
                                        ),
                                    ),
                                    initial_value: 14,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 17,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternExprRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `N`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `ct_start`,
                                            regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `ct_end`,
                                            regional_token_idx: RegionalTokenIdx(
                                                23,
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
                                ],
                            },
                            pattern_symbol_arena: Arena {
                                data: [
                                    SynPatternSymbol::Atom(
                                        0,
                                    ),
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
                                        `N`,
                                        0,
                                    ),
                                ],
                                [
                                    (
                                        `ct_start`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `ct_end`,
                                        2,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Pure,
                                    Pure,
                                    Pure,
                                ],
                            },
                        },
                        symbol_region: VariableRegionData {
                            inherited_syn_symbol_arena: Arena {
                                data: [
                                    InheritedVariable {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::ParenateParameter {
                                            ident: `start`,
                                        },
                                    },
                                    InheritedVariable {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::ParenateParameter {
                                            ident: `end`,
                                        },
                                    },
                                ],
                            },
                            current_syn_symbol_arena: Arena {
                                data: [
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            3,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    39,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `N`,
                                            pattern_symbol_idx: 0,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            13,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    39,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `ct_start`,
                                            pattern_symbol_idx: 1,
                                        },
                                    },
                                    CurrentSynSymbolEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            24,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    39,
                                                ),
                                            ),
                                        ),
                                        data: CurrentSynSymbolData::LetVariable {
                                            ident: `ct_end`,
                                            pattern_symbol_idx: 2,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: True,
                            allow_self_value: True,
                            pattern_ty_constraints: [],
                        },
                        syn_pattern_expr_roots: [
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 0,
                            },
                            SynPatternRoot {
                                kind: SynPatternExprRootKind::Let,
                                syn_pattern_expr_idx: 1,
                            },
                            SynPatternRoot {
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
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 8,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 14,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 17,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::BlockExpr,
                                syn_expr_idx: 18,
                            },
                        ],
                        has_self_lifetime: false,
                        has_self_place: false,
                        syn_pattern_to_current_syn_symbol_map: [
                            (
                                0,
                                0,
                            ),
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
```