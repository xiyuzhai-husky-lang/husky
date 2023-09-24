Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Val(
                    ValSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                            ident: `simple_leftdown_pattern`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                10,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                                    value: 96,
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
                                                        16,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 7,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    opd: 8,
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
                                                    expr_idx: 9,
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
                                                                    19,
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
                                                expr_idx: 9,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 10,
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
                        path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        9,
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
                                                            expr_idx: 5,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        10,
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
                                                            ident: `leftupcc_pattern`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `leftdowncc_pattern`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmt::Eval {
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
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 7,
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
                        path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                                                                        path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                                                                    value: 97,
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
                                                        ident: `ymax`,
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
                                                                    value: 98,
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
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                    },
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                20,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                                    value: 99,
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
                                                                    value: 100,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 9,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    ropd: 10,
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start_tangent`,
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
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        36,
                                                    ),
                                                    Literal::Bool(
                                                        True,
                                                    ),
                                                ),
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 14,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ang`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        41,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 101,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 16,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        40,
                                                    ),
                                                    ropd: 17,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `ang`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..7,
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
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            25,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            pattern_expr_idx: 2,
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                27,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 15,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                    condition: 18,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 19,
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
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `ang`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                26,
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
                                            pattern_infos: [
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
                                                        `ang`,
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
                                                                    43,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `dp`,
                                                            pattern_symbol_idx: 1,
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
                                                                    43,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `ang`,
                                                            pattern_symbol_idx: 2,
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
                                                kind: LetStmtInitialValue,
                                                expr_idx: 15,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 18,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 19,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 20,
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
                        path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                        decl: ValSynDecl {
                            path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                                            ident: `Seven`,
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
                                                        ident: `Seven`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Seven`,
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
                                61,
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
                                                                        path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                                                            ident: `Seven`,
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
                                                                        ident: `Seven`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            12,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::TypeVariant(
                                                                        TypeVariantPath {
                                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                            ident: `Seven`,
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                                        11,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max_hole_ilen`,
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
                                                                    value: 102,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 6,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    ropd: 7,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `simple_match_norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 103,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 11,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    ropd: 12,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 5,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 14,
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
                                                    owner: 15,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 16,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        32,
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 17,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        33,
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
                                                    item_path_expr: 6,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 7,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 19,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            40,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 20,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            44,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 21,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        41,
                                                    ),
                                                    ropd: 22,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `upper_excess`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        48,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 104,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 24,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        47,
                                                    ),
                                                    ropd: 25,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 8,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 27,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            55,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        57,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 28,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        56,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 29,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        58,
                                                    ),
                                                },
                                                SynExpr::Suffix {
                                                    opd: 30,
                                                    opr: UnwrapOrComposeWithNot,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        59,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 31,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        60,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end_tangent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            61,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        62,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        63,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `end_tangent`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        67,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 33,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        68,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `y`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        74,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 105,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::CurrentSymbol {
                                                    ident: `a`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        71,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                SynExpr::Prefix {
                                                    opr: Minus,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        73,
                                                    ),
                                                    opd: 35,
                                                },
                                                SynExpr::Binary {
                                                    lopd: 36,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        72,
                                                    ),
                                                    ropd: 37,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 10,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ident: `Yes`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `simple_match_norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        80,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        82,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 106,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 40,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        81,
                                                    ),
                                                    ropd: 41,
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
                                                SynExpr::Field {
                                                    owner: 43,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        88,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `upper_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            89,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Field {
                                                    owner: 44,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        92,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `lower_mass`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            93,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Binary {
                                                    lopd: 45,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        90,
                                                    ),
                                                    ropd: 46,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `upper_excess`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        95,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        97,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 107,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::Binary {
                                                    lopd: 48,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        96,
                                                    ),
                                                    ropd: 49,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 14,
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
                                                    item_path_expr: 15,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 52,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        104,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `matches`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            105,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        107,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            0,
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 53,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        106,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 54,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        108,
                                                    ),
                                                },
                                                SynExpr::Be {
                                                    src: 55,
                                                    be_regional_token_idx: RegionalTokenIdx(
                                                        109,
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
                                                    item_path_expr: 16,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Fugitive(
                                                                FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 57,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        115,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            116,
                                                        ),
                                                    },
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        118,
                                                    ),
                                                    Literal::Bool(
                                                        False,
                                                    ),
                                                ),
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
                                                        11..21,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `is_six`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                2,
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
                                                            ident: `is_zero`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
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
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
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
                                                            ident: `simple_seven_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `simple_seven_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                27,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                38,
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
                                                                42,
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
                                                            ident: `simple_seven_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                53,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                76,
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
                                                    parent: 9,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            77,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                78,
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
                                                            ident: `major_connected_component`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                87,
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
                                                                91,
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
                                                                99,
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
                                                    parent: 13,
                                                    colon_colon_token: ColonColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            100,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                101,
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
                                                            ident: `special_seven_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                103,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `special_seven_match`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                114,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `OneVsAll`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                119,
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
                                                            120,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentRegionalToken {
                                                            ident: `Yes`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                121,
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
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            50,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetPatternObelisk {
                                                            pattern_expr_idx: 6,
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                52,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 32,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            64,
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
                                                                66,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 34,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            70,
                                                        ),
                                                    },
                                                    condition: 38,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            26,
                                                        ),
                                                    },
                                                    condition: 18,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            35,
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
                                                                37,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 23,
                                                },
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                45,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            26,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        49,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            1..4,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            75,
                                                        ),
                                                    },
                                                    result: 39,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            84,
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
                                                                86,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 47,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            94,
                                                        ),
                                                    },
                                                    condition: 50,
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            98,
                                                        ),
                                                    },
                                                    result: 51,
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
                                                    condition: 8,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
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
                                                                17,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 10,
                                                },
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            13,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        25,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            4..8,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmt::IfElse {
                                                    if_branch: SynIfBranch {
                                                        if_token: IfRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                79,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            42,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        83,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            8..11,
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            102,
                                                        ),
                                                    },
                                                    condition: 56,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            111,
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
                                                                113,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 58,
                                                },
                                                SynStmt::Require {
                                                    require_token: RequireRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                    condition: 59,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 60,
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
                                                            ident: `simple_match_norm`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `upper_excess`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                36,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `end_tangent`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                51,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `a`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                65,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `upper_excess`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                85,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `some`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                110,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `others`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                112,
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
                                                        `simple_match_norm`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
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
                                                        `end_tangent`,
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
                                                        `upper_excess`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `others`,
                                                        10,
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
                                                                    122,
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
                                                                    122,
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
                                                            17,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    122,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `simple_match_norm`,
                                                            pattern_symbol_idx: 3,
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
                                                                    79,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 4,
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
                                                                    79,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `upper_excess`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            52,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    75,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `end_tangent`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            66,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    75,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            86,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    102,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `upper_excess`,
                                                            pattern_symbol_idx: 8,
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
                                                                    122,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::BeVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            113,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    122,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 10,
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
                                                expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 10,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 18,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 23,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 32,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 34,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 38,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 39,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 47,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 50,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 51,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 56,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 58,
                                            },
                                            SynExprRoot {
                                                kind: Condition,
                                                expr_idx: 59,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 60,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 61,
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