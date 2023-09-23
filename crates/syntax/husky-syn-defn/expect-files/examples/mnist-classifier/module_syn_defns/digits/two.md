Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                8,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 5,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 4,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 5,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        11,
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
                                                            expr_idx: 6,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        12,
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
                                                            ident: `left_cc_pattern`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
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
                                                            FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
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
                                                            FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
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
                                                expr_idx: 7,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 8,
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
                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
                                                                    value: 119,
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
                                                        LetVariableObelisk {
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
                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
                                                                    value: 120,
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
                                                        LetVariableObelisk {
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
                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                        ident: `x`,
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
                                                                    value: 121,
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
                                                        ident: `x`,
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
                                                        LetVariableObelisk {
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
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                            ident: `Two`,
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
                                                        ident: `Two`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Two`,
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
                                122,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                                            ident: `Two`,
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
                                                                        ident: `Two`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::TypeVariant(
                                                                        TypeVariantPath {
                                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                            ident: `Two`,
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
                                                            path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                        BeVariablesObelisk {
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
                                                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
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
                                                        BeVariablesObelisk {
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
                                                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 5,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 3,
                                                            variables: ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 7,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 4,
                                                            variables: ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 5,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 9,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 5,
                                                            variables: ArenaIdxRange(
                                                                5..6,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 6,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 11,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 6,
                                                            variables: ArenaIdxRange(
                                                                6..7,
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
                                                    self_argument: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            30,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 8,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 15,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `eff_holes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 17,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        41,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            42,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        44,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 18,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 19,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        45,
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 20,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 9,
                                                            variables: ArenaIdxRange(
                                                                9..10,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 9,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        55,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 23,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 24,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        56,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 10,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 26,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        61,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        64,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 27,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        63,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 28,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        65,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 11,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 30,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        70,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            71,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        73,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 31,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        72,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 32,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        74,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `cc_num`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        76,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        78,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 34,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        77,
                                                    ),
                                                    ropd: 35,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 12,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 13,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 37,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        83,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            84,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 38,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        87,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            88,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 39,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        85,
                                                    ),
                                                    ropd: 40,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `lower_excess`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        90,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        92,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 122,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 42,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        91,
                                                    ),
                                                    ropd: 43,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `cc_num`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        94,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        96,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 45,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        95,
                                                    ),
                                                    ropd: 46,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        99,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 48,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        100,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 14,
                                                            variables: ArenaIdxRange(
                                                                14..15,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        103,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 50,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        104,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 15,
                                                            variables: ArenaIdxRange(
                                                                15..16,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        109,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 52,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        110,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 53,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        111,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            112,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        117,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 123,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::CurrentSymbol {
                                                    ident: `a`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        114,
                                                    ),
                                                    current_symbol_idx: 16,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                SynExpr::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        116,
                                                    ),
                                                    opd: 55,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 56,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        115,
                                                    ),
                                                    ropd: 57,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        121,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 59,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        122,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 60,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        123,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end_tangent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            124,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        125,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        126,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        130,
                                                    ),
                                                    Literal::Bool(
                                                        True,
                                                    ),
                                                ),
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 61,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        127,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            128,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        129,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 62,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        131,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        135,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 64,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        136,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 65,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        137,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end_tangent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        139,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        140,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 66,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        141,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `x`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            142,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        146,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 68,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        147,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 69,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        148,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end_tangent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            149,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        150,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        151,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 70,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        152,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            153,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        157,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 72,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        158,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 73,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        159,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            160,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 74,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        161,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymax`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            162,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        163,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        164,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        168,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 76,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        169,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 77,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        170,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            171,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 78,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        172,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymin`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            173,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        174,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        175,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_ymax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        180,
                                                    ),
                                                    current_symbol_idx: 20,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 20,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_ymin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        182,
                                                    ),
                                                    current_symbol_idx: 21,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 21,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 80,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        181,
                                                    ),
                                                    ropd: 81,
                                                },
                                                SynExpr::Bracketed {
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        179,
                                                    ),
                                                    item: 82,
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        183,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        185,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 124,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 83,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        184,
                                                    ),
                                                    ropd: 84,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        189,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 86,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        190,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 87,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        191,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            192,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 88,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        193,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymax`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            194,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        195,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        196,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        200,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 90,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        201,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 91,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        202,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `relative_bounding_box`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            203,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 92,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        204,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymin`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            205,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        206,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        207,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_ymax`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        212,
                                                    ),
                                                    current_symbol_idx: 23,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 23,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_ymin`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        214,
                                                    ),
                                                    current_symbol_idx: 24,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 24,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 94,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        213,
                                                    ),
                                                    ropd: 95,
                                                },
                                                SynExpr::Bracketed {
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        211,
                                                    ),
                                                    item: 96,
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        215,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        217,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 125,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 97,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        216,
                                                    ),
                                                    ropd: 98,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_mid_y`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        219,
                                                    ),
                                                    current_symbol_idx: 22,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 22,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_mid_y`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        221,
                                                    ),
                                                    current_symbol_idx: 25,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 25,
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 100,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        220,
                                                    ),
                                                    ropd: 101,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `cc_num`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        223,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        225,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 103,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        224,
                                                    ),
                                                    ropd: 104,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        228,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 106,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        229,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 26,
                                                            variables: ArenaIdxRange(
                                                                26..27,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        232,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 108,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        233,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 27,
                                                            variables: ArenaIdxRange(
                                                                27..28,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `down_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        236,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                SynExpr::Be {
                                                    src: 110,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        237,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesObelisk {
                                                            pattern_expr: 28,
                                                            variables: ArenaIdxRange(
                                                                28..29,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `down_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        240,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 112,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        241,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 113,
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
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 114,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        244,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ymin`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            245,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        246,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        247,
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        249,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 126,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 115,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        248,
                                                    ),
                                                    ropd: 116,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `down_cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        253,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                SynExpr::Suffix {
                                                    opd: 118,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        254,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 119,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        255,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            256,
                                                        ),
                                                    },
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 15,
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
                                                        20..38,
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
                                                            ident: `is_three`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_seven`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_one`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_nine`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_six`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
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
                                                            ident: `major_concave_components`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                28,
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
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                36,
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
                                                            ident: `two_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                51,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `two_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                60,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `two_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                69,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                82,
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
                                                                86,
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
                                                                257,
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
                                                    parent: 14,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            258,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                259,
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
                                                            98,
                                                        ),
                                                    },
                                                    condition: 49,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            102,
                                                        ),
                                                    },
                                                    condition: 51,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            106,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 16,
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                108,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 54,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            113,
                                                        ),
                                                    },
                                                    condition: 58,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            118,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 17,
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                120,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 63,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            132,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 18,
                                                            variables: ArenaIdxRange(
                                                                18..19,
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
                                                                134,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 67,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            143,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 19,
                                                            variables: ArenaIdxRange(
                                                                19..20,
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
                                                                145,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 71,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            154,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 20,
                                                            variables: ArenaIdxRange(
                                                                20..21,
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
                                                                156,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 75,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            165,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 21,
                                                            variables: ArenaIdxRange(
                                                                21..22,
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
                                                                167,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 79,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            176,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 22,
                                                            variables: ArenaIdxRange(
                                                                22..23,
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
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 85,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            186,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 23,
                                                            variables: ArenaIdxRange(
                                                                23..24,
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
                                                                188,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 89,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 24,
                                                            variables: ArenaIdxRange(
                                                                24..25,
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
                                                                199,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 93,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            208,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 25,
                                                            variables: ArenaIdxRange(
                                                                25..26,
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
                                                                210,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 99,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            218,
                                                        ),
                                                    },
                                                    condition: 102,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            227,
                                                        ),
                                                    },
                                                    condition: 107,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            231,
                                                        ),
                                                    },
                                                    condition: 109,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            235,
                                                        ),
                                                    },
                                                    condition: 111,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            239,
                                                        ),
                                                    },
                                                    condition: 117,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            250,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 29,
                                                            variables: ArenaIdxRange(
                                                                29..30,
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
                                                                252,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 120,
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
                                                            13,
                                                        ),
                                                    },
                                                    condition: 8,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                    condition: 10,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            21,
                                                        ),
                                                    },
                                                    condition: 12,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            25,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
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
                                                                27,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 14,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            33,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
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
                                                                35,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 16,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            39,
                                                        ),
                                                    },
                                                    condition: 21,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            48,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
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
                                                                50,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 25,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            57,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 11,
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                59,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 29,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            66,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
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
                                                                68,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 33,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            75,
                                                        ),
                                                    },
                                                    condition: 36,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            79,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
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
                                                                81,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 41,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            89,
                                                        ),
                                                    },
                                                    condition: 44,
                                                },
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                93,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            47,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        97,
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
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                222,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            105,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        226,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            15..20,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 121,
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
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                20,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                24,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `cc_num`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                26,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `eff_holes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `none`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                47,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `left_cc`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                49,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `right_cc`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                58,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `down_cc`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                67,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `lower_excess`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                80,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                101,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                105,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `a`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                107,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `end_tan`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                119,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `x`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                133,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `y`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                144,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `left_ymax`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                155,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `left_ymin`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                166,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `left_mid_y`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                177,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `right_ymax`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                187,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `right_ymin`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                198,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `right_mid_y`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                209,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                230,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                234,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                238,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `a`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                251,
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
                                                        23,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        24,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        25,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        26,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        27,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        28,
                                                    ),
                                                    SynPatternSymbol::Atom(
                                                        29,
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
                                                        `none`,
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
                                                        `cc_num`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `eff_holes`,
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
                                                        `left_cc`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_cc`,
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `down_cc`,
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `lower_excess`,
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
                                                        `some`,
                                                        15,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        16,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `end_tan`,
                                                        17,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `x`,
                                                        18,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `y`,
                                                        19,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `left_ymax`,
                                                        20,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `left_ymin`,
                                                        21,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `left_mid_y`,
                                                        22,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_ymax`,
                                                        23,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_ymin`,
                                                        24,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_mid_y`,
                                                        25,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        26,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        27,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        28,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        29,
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
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
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
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
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
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
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
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            21,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            25,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `cc_num`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            35,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `eff_holes`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            48,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 9,
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
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `left_cc`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            59,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `right_cc`,
                                                            pattern_symbol_idx: 11,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            68,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `down_cc`,
                                                            pattern_symbol_idx: 12,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            81,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    260,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `lower_excess`,
                                                            pattern_symbol_idx: 13,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            102,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 14,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            106,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 15,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            108,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 16,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            120,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `end_tan`,
                                                            pattern_symbol_idx: 17,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            134,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `x`,
                                                            pattern_symbol_idx: 18,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            145,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `y`,
                                                            pattern_symbol_idx: 19,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            156,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `left_ymax`,
                                                            pattern_symbol_idx: 20,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            167,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `left_ymin`,
                                                            pattern_symbol_idx: 21,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            178,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `left_mid_y`,
                                                            pattern_symbol_idx: 22,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            188,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `right_ymax`,
                                                            pattern_symbol_idx: 23,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            199,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `right_ymin`,
                                                            pattern_symbol_idx: 24,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            210,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    222,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `right_mid_y`,
                                                            pattern_symbol_idx: 25,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            231,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 26,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            235,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 27,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            239,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 28,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            252,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    257,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 29,
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
                                                kind: Condition,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 10,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 12,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 14,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 16,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 21,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 25,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 29,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 33,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 36,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 41,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 44,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 49,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 51,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 54,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 58,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 63,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 67,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 71,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 75,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 79,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 85,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 89,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 93,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 99,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 102,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 107,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 109,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 111,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 117,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 120,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 121,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 122,
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