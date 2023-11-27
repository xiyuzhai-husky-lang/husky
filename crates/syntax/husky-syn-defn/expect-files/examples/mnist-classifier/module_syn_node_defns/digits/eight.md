[
    ItemSynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::Val(
                ValSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: ValFugitiveSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        colon_token: Ok(
                            Some(
                                ColonRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 1,
                                },
                            ),
                        ),
                        eq_token: Ok(
                            EqRegionalToken(
                                RegionalTokenIdx(
                                    5,
                                ),
                            ),
                        ),
                        expr: None,
                        syn_expr_region: SynExprRegion {
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
                                                                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                            6,
                            SynExprRegion {
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
                                                                                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                                            FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
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
                                                        ident: `big_mouth`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
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
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::Val(
                ValSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: ValFugitiveSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        colon_token: Ok(
                            Some(
                                ColonRegionalToken(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeEqSyndicate {
                                    expr: 5,
                                },
                            ),
                        ),
                        eq_token: Ok(
                            EqRegionalToken(
                                RegionalTokenIdx(
                                    13,
                                ),
                            ),
                        ),
                        expr: None,
                        syn_expr_region: SynExprRegion {
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
                                                                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                                    TypeVariantPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 321,
                                                            },
                                                        ),
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
                                                    ident: `Eight`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 321,
                                                            },
                                                        ),
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
                    },
                    body_with_syn_expr_region: Some(
                        (
                            29,
                            SynExprRegion {
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
                                                                                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                                                    TypeVariantPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 321,
                                                                            },
                                                                        ),
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
                                                                    ident: `Eight`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::TypeVariant(
                                                                    TypeVariantPath(
                                                                        ItemPathId(
                                                                            Id {
                                                                                value: 321,
                                                                            },
                                                                        ),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                                            FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Be {
                                                src: 3,
                                                be_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                target: Ok(
                                                    BePatternSynSyndicate {
                                                        pattern_expr_root: BeSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 2,
                                                        },
                                                        variables: ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    },
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
                                            SynExprData::Be {
                                                src: 5,
                                                be_regional_token_idx: RegionalTokenIdx(
                                                    11,
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
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Be {
                                                src: 7,
                                                be_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                target: Ok(
                                                    BePatternSynSyndicate {
                                                        pattern_expr_root: BeSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 4,
                                                        },
                                                        variables: ArenaIdxRange(
                                                            4..5,
                                                        ),
                                                    },
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
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 6,
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
                                                    21,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `upper_mass`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 10,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `lower_mass`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 11,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    23,
                                                ),
                                                ropd: 12,
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
                                                owner: 14,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    29,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 15,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
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
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 16,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    33,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 17,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    35,
                                                ),
                                            },
                                            SynExprData::Be {
                                                src: 18,
                                                be_regional_token_idx: RegionalTokenIdx(
                                                    36,
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
                                                owner: 20,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    41,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 21,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    43,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        44,
                                                    ),
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    46,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 22,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    45,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 23,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    47,
                                                ),
                                            },
                                            SynExprData::Be {
                                                src: 24,
                                                be_regional_token_idx: RegionalTokenIdx(
                                                    48,
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
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    52,
                                                ),
                                                LiteralData::Bool(
                                                    False,
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    54,
                                                ),
                                                LiteralData::Bool(
                                                    False,
                                                ),
                                            ),
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 10,
                                                opt_path: Some(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 303,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    4..11,
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
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `is_zero`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
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
                                                            14,
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
                                                            24,
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
                                                            28,
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
                                                            40,
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
                                                            55,
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
                                                parent: 9,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        56,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `Yes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            57,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 303,
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
                                            SynStmtData::Require {
                                                require_token: RequireRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        51,
                                                    ),
                                                },
                                                condition: 26,
                                            },
                                            SynStmtData::IfElse {
                                                if_branch: SynIfBranch {
                                                    if_token: IfRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            39,
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
                                            SynStmtData::Require {
                                                require_token: RequireRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        53,
                                                    ),
                                                },
                                                condition: 27,
                                            },
                                            SynStmtData::Require {
                                                require_token: RequireRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        1,
                                                    ),
                                                },
                                                condition: 2,
                                            },
                                            SynStmtData::Require {
                                                require_token: RequireRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                condition: 4,
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
                                                        13,
                                                    ),
                                                },
                                                condition: 8,
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
                                                            19,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 13,
                                            },
                                            SynStmtData::IfElse {
                                                if_branch: SynIfBranch {
                                                    if_token: IfRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        19,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                38,
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
                                                expr_idx: 28,
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
                                                        ident: `none`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `none`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `none`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_excess`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `none`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            37,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `none`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            49,
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
                                                    `none`,
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
                                                    `upper_excess`,
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
                                                    `none`,
                                                    7,
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
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                58,
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
                                                        9,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                58,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::BeVariable {
                                                        ident: `none`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                58,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::BeVariable {
                                                        ident: `none`,
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                58,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::BeVariable {
                                                        ident: `none`,
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                58,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
                                                        ident: `upper_excess`,
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        38,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                55,
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
                                                        50,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                53,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::BeVariable {
                                                        ident: `none`,
                                                        pattern_symbol_idx: 7,
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
                                            kind: SynPatternExprRootKind::Be,
                                            syn_pattern_expr_idx: 2,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Be,
                                            syn_pattern_expr_idx: 3,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Be,
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
                                            syn_pattern_expr_idx: 7,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::Condition,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::Condition,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::Condition,
                                            syn_expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::Condition,
                                            syn_expr_idx: 8,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 13,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::Condition,
                                            syn_expr_idx: 26,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::Condition,
                                            syn_expr_idx: 27,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 28,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 29,
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
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemSynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::FunctionFn(
                FnSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    syn_node_decl: FnSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_obelisk_list: Ok(
                            None,
                        ),
                        parenate_parameter_obelisk_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
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
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 4,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
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
                                                                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
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
                            22,
                            SynExprRegion {
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
                                                                                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
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
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
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
                                    expr_arena: Arena {
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
                                                    ident: `relative_bounding_box`,
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
                                                    ident: `ymax`,
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
                                                LiteralData::Float(
                                                    Unspecified(
                                                        UnspecifiedFloatLiteral(
                                                            Id {
                                                                value: 108,
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
                                            SynExprData::InheritedSynSymbol {
                                                ident: `cc`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 6,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `strokes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
                                                self_argument: 7,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `first`,
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
                                            SynExprData::Suffix {
                                                opd: 8,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    20,
                                                ),
                                            },
                                            SynExprData::Field {
                                                owner: 9,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                },
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `cc`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    26,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 11,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    27,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `strokes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
                                                self_argument: 12,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    29,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `first`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                            },
                                            SynExprData::Suffix {
                                                opd: 13,
                                                opr: UnwrapOrComposeWithNot,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    33,
                                                ),
                                            },
                                            SynExprData::Field {
                                                owner: 14,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    34,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 10,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    23,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `x`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 15,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    36,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `x`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 16,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                                ropd: 17,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `cc`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    38,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                            SynExprData::Field {
                                                owner: 19,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    39,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `relative_bounding_box`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
                                                self_argument: 20,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    41,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `ymax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                },
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    43,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    44,
                                                ),
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    2..4,
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
                                                        12,
                                                    ),
                                                },
                                                condition: 18,
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
                                                        1..2,
                                                    ),
                                                },
                                                elif_branches: [],
                                                else_branch: None,
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 21,
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
                                                    modifier: None,
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
                                            syn_expr_idx: 18,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 21,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 22,
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
]